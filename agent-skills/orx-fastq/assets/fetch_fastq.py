#!/usr/bin/env python3
"""Download public FASTQ files for a BioProject, study, sample, run, or GEO accession.

Resolves runs through the ENA Portal API (which mirrors SRA and DDBJ), downloads
over HTTPS with resume and MD5 verification, and writes:

  <out>/<run>_1.fastq.gz, <run>_2.fastq.gz   paired-end (ExomeFlow's naming)
  <out>/<run>.fastq.gz                        single-end
  <out>/runinfo.tsv                           run metadata + download status
  <out>/samplesheet.csv                       nf-core style: sample,fastq_1,fastq_2,strandedness

Standard library only (Python 3.9+). Examples:

  python3 fetch_fastq.py PRJNA257197 --list
  python3 fetch_fastq.py GSE52778 --out data/fastq --parallel 4
  python3 fetch_fastq.py PRJEB12345 --runs ERR111,ERR112 --out data/fastq
"""

from __future__ import annotations

import argparse
import concurrent.futures
import csv
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import time
import urllib.error
import urllib.parse
import urllib.request

ENA_FILEREPORT = "https://www.ebi.ac.uk/ena/portal/api/filereport"
NCBI_EUTILS = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils"
USER_AGENT = "pg-lab-ai-fetch-fastq/1.0 (+https://github.com/imrobintomar/PG-Lab-AI)"
FIELDS = [
    "run_accession", "study_accession", "secondary_study_accession", "sample_accession",
    "experiment_accession", "sample_alias", "sample_title", "experiment_title",
    "library_layout", "library_strategy", "library_source", "instrument_platform",
    "instrument_model", "read_count", "base_count", "scientific_name",
    "fastq_ftp", "fastq_md5", "fastq_bytes",
]
ACCESSION = re.compile(
    r"^(PRJ[EDN][A-Z]\d+|[EDS]R[PSXR]\d+|SAM[EDN][A-Z]?\d+|GS[EM]\d+)$", re.IGNORECASE
)
CHUNK = 1 << 20


def http_get(url: str, params: dict | None = None, retries: int = 4) -> bytes:
    if params:
        url = f"{url}?{urllib.parse.urlencode(params)}"
    for attempt in range(1, retries + 1):
        try:
            req = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
            with urllib.request.urlopen(req, timeout=60) as resp:
                return resp.read()
        except urllib.error.HTTPError as err:
            if err.code in (400, 404) or attempt == retries:
                raise
        except (urllib.error.URLError, TimeoutError):
            if attempt == retries:
                raise
        time.sleep(2 ** attempt)
    raise RuntimeError("unreachable")


def ena_runs(accession: str) -> list[dict]:
    body = http_get(
        ENA_FILEREPORT,
        {"accession": accession, "result": "read_run", "fields": ",".join(FIELDS),
         "format": "tsv", "limit": 0},
    ).decode()
    rows = list(csv.DictReader(body.splitlines(), delimiter="\t"))
    return [r for r in rows if r.get("run_accession")]


def geo_runs(accession: str) -> list[dict]:
    """GEO links each series to an SRA study and each sample to an SRA experiment;
    ENA resolves those to runs and files."""
    email = os.environ.get("NCBI_EMAIL", "drprabudhgoel@gmail.com")
    common = {"tool": "pg-lab-ai", "email": email, "retmode": "json"}
    entry_type = "gse" if accession.startswith("GSE") else "gsm"
    found = json.loads(http_get(
        f"{NCBI_EUTILS}/esearch.fcgi",
        {**common, "db": "gds", "term": f"{accession}[ACCN] AND {entry_type}[ETYP]"},
    ))
    ids = found.get("esearchresult", {}).get("idlist", [])
    if not ids:
        return []
    time.sleep(0.4)  # NCBI allows 3 requests/second without an API key
    summary = json.loads(http_get(f"{NCBI_EUTILS}/esummary.fcgi", {**common, "db": "gds", "id": ",".join(ids)}))
    result = summary.get("result", {})
    targets = sorted({
        rel["targetobject"]
        for uid in result.get("uids", [])
        for rel in result[uid].get("extrelations", [])
        if rel.get("relationtype") == "SRA" and rel.get("targetobject")
    })
    rows: list[dict] = []
    for target in targets:
        rows += ena_runs(target)
    return rows


def resolve(accession: str) -> list[dict]:
    if not ACCESSION.match(accession):
        raise SystemExit(
            f"{accession!r} is not a recognised accession. Use a BioProject (PRJNA…/PRJEB…/PRJDB…), "
            "study (SRP/ERP/DRP), sample (SRS/ERS/SAMN/SAMEA), experiment (SRX/ERX), "
            "run (SRR/ERR/DRR), or GEO series/sample (GSE/GSM)."
        )
    accession = accession.upper()
    rows = geo_runs(accession) if accession.startswith("GS") else ena_runs(accession)
    if not rows:
        raise SystemExit(f"No sequencing runs found for {accession}. It may be private, embargoed, or not yet mirrored to ENA.")
    return rows


def files_for(row: dict) -> list[dict]:
    """The FASTQ files to fetch for one run, named the way ExomeFlow expects."""
    urls = [u for u in row.get("fastq_ftp", "").split(";") if u]
    md5s = row.get("fastq_md5", "").split(";")
    sizes = row.get("fastq_bytes", "").split(";")
    entries = [
        {"url": "https://" + u.split("://")[-1], "name": u.rsplit("/", 1)[-1],
         "md5": md5s[i] if i < len(md5s) else "", "bytes": int(sizes[i]) if i < len(sizes) and sizes[i].isdigit() else 0}
        for i, u in enumerate(urls)
    ]
    run = row["run_accession"]
    paired = [e for e in entries if re.search(rf"{run}_[12]\.fastq\.gz$", e["name"])]
    if row.get("library_layout", "").upper() == "PAIRED" and len(paired) == 2:
        return sorted(paired, key=lambda e: e["name"])
    return entries


def human(n: float) -> str:
    for unit in ("B", "KB", "MB", "GB", "TB"):
        if n < 1024 or unit == "TB":
            return f"{n:.1f} {unit}"
        n /= 1024
    return f"{n:.1f} TB"


def md5sum(path: str) -> str:
    digest = hashlib.md5()
    with open(path, "rb") as fh:
        for block in iter(lambda: fh.read(CHUNK), b""):
            digest.update(block)
    return digest.hexdigest()


def download(entry: dict, out: str, retries: int = 5) -> str:
    target = os.path.join(out, entry["name"])
    if os.path.exists(target) and (not entry["md5"] or md5sum(target) == entry["md5"]):
        return "present"
    part = target + ".part"
    for attempt in range(1, retries + 1):
        try:
            have = os.path.getsize(part) if os.path.exists(part) else 0
            headers = {"User-Agent": USER_AGENT}
            if have:
                headers["Range"] = f"bytes={have}-"
            req = urllib.request.Request(entry["url"], headers=headers)
            with urllib.request.urlopen(req, timeout=120) as resp, open(part, "ab" if have and resp.status == 206 else "wb") as fh:
                shutil.copyfileobj(resp, fh, CHUNK)
            if entry["md5"] and md5sum(part) != entry["md5"]:
                os.remove(part)
                raise IOError("MD5 mismatch")
            os.replace(part, target)
            return "downloaded"
        except (urllib.error.URLError, IOError, TimeoutError) as err:
            if attempt == retries:
                return f"failed: {err}"
            time.sleep(min(60, 2 ** attempt))
    return "failed"


def sra_fallback(run: str, out: str) -> str:
    """Runs ENA has no FASTQ for: convert with sra-tools when it is installed."""
    if not shutil.which("fasterq-dump"):
        return "no ENA FASTQ; install sra-tools (fasterq-dump) or rerun with --sra-fallback"
    result = subprocess.run(
        ["fasterq-dump", "--split-files", "--outdir", out, run], capture_output=True, text=True
    )
    if result.returncode != 0:
        return f"failed: fasterq-dump {result.stderr.strip()[:200]}"
    gz = shutil.which("pigz") or shutil.which("gzip")
    for name in os.listdir(out):
        if name.startswith(run) and name.endswith(".fastq") and gz:
            subprocess.run([gz, os.path.join(out, name)], check=False)
    return "converted with fasterq-dump"


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("accessions", nargs="+", help="BioProject, study, sample, experiment, run, or GEO accession(s)")
    ap.add_argument("--out", default="fastq", help="output directory (default: fastq)")
    ap.add_argument("--list", action="store_true", help="only list runs, files, and total size")
    ap.add_argument("--runs", help="comma-separated run accessions to keep")
    ap.add_argument("--match", help="regex on sample alias/title or experiment title to keep")
    ap.add_argument("--strategy", help="keep one library strategy, e.g. RNA-Seq, WXS, WGS")
    ap.add_argument("--max-runs", type=int, help="keep at most this many runs")
    ap.add_argument("--parallel", type=int, default=3, help="concurrent downloads (default: 3)")
    ap.add_argument("--confirm-above-gb", type=float, default=20.0,
                    help="refuse downloads larger than this without --yes (default: 20)")
    ap.add_argument("--yes", action="store_true", help="confirm a large download")
    ap.add_argument("--sra-fallback", action="store_true", help="use fasterq-dump for runs ENA has no FASTQ for")
    args = ap.parse_args()

    rows: list[dict] = []
    for accession in args.accessions:
        rows += resolve(accession.strip())
    seen: set[str] = set()
    rows = [r for r in rows if not (r["run_accession"] in seen or seen.add(r["run_accession"]))]
    if args.runs:
        keep = {r.strip().upper() for r in args.runs.split(",")}
        rows = [r for r in rows if r["run_accession"].upper() in keep]
    if args.match:
        pattern = re.compile(args.match, re.IGNORECASE)
        rows = [r for r in rows if pattern.search(" ".join(r.get(k, "") for k in ("sample_alias", "sample_title", "experiment_title")))]
    if args.strategy:
        rows = [r for r in rows if r.get("library_strategy", "").lower() == args.strategy.lower()]
    if args.max_runs:
        rows = rows[: args.max_runs]
    if not rows:
        raise SystemExit("No runs left after filtering.")

    total = 0
    print(f"{'run':<14}{'layout':<8}{'strategy':<12}{'files':>5}{'size':>11}  title")
    for row in rows:
        files = files_for(row)
        size = sum(f["bytes"] for f in files)
        total += size
        title = row.get("sample_title") or row.get("sample_alias") or row.get("experiment_title") or ""
        print(f"{row['run_accession']:<14}{row.get('library_layout', ''):<8}{row.get('library_strategy', ''):<12}"
              f"{len(files):>5}{human(size):>11}  {title[:60]}")
    print(f"\n{len(rows)} runs, {human(total)} of compressed FASTQ "
          f"({', '.join(sorted({r.get('scientific_name', '') for r in rows} - {''}))})")
    if args.list:
        return

    os.makedirs(args.out, exist_ok=True)
    free = shutil.disk_usage(args.out).free
    if total > free * 0.9:
        raise SystemExit(f"Not enough disk space in {args.out}: need {human(total)}, {human(free)} free.")
    if total > args.confirm_above_gb * 1024 ** 3 and not args.yes:
        raise SystemExit(f"This downloads {human(total)}. Confirm with the user, then rerun with --yes.")

    jobs = [(row, f) for row in rows for f in files_for(row)]
    status: dict[str, list[str]] = {r["run_accession"]: [] for r in rows}
    with concurrent.futures.ThreadPoolExecutor(max_workers=max(1, args.parallel)) as pool:
        futures = {pool.submit(download, f, args.out): (row, f) for row, f in jobs}
        for done, future in enumerate(concurrent.futures.as_completed(futures), 1):
            row, f = futures[future]
            result = future.result()
            status[row["run_accession"]].append(result)
            print(f"[{done}/{len(jobs)}] {f['name']}: {result}", flush=True)
    for row in rows:
        if not files_for(row):
            status[row["run_accession"]].append(
                sra_fallback(row["run_accession"], args.out) if args.sra_fallback
                else "no ENA FASTQ; rerun with --sra-fallback (needs sra-tools)")

    with open(os.path.join(args.out, "runinfo.tsv"), "w", newline="") as fh:
        writer = csv.DictWriter(fh, fieldnames=FIELDS + ["download_status"], delimiter="\t", extrasaction="ignore")
        writer.writeheader()
        for row in rows:
            writer.writerow({**row, "download_status": "; ".join(status[row["run_accession"]])})
    with open(os.path.join(args.out, "samplesheet.csv"), "w", newline="") as fh:
        writer = csv.writer(fh)
        writer.writerow(["sample", "fastq_1", "fastq_2", "strandedness"])
        for row in rows:
            names = [os.path.abspath(os.path.join(args.out, f["name"])) for f in files_for(row)]
            if names:
                writer.writerow([row["run_accession"], names[0], names[1] if len(names) > 1 else "", "auto"])

    failed = [run for run, s in status.items() if any(x.startswith(("failed", "no ENA")) for x in s)]
    print(f"\nWrote {args.out}/runinfo.tsv and {args.out}/samplesheet.csv")
    if failed:
        print(f"{len(failed)} run(s) incomplete: {', '.join(failed)} — see runinfo.tsv", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
