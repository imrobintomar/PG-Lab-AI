---
name: orx-fastq
description: "Download public sequencing reads (FASTQ) for a BioProject, SRA/ENA/DDBJ study, sample, experiment, run, or GEO series/sample accession, with checksums and a samplesheet. Use when the user gives a project ID or accession (PRJNA…, PRJEB…, SRP…, SRR…, ERR…, GSE…, GSM…) or asks to fetch, reanalyze, or reproduce public sequencing data."
---

# Download FASTQ from a project accession

Use the bundled downloader. It resolves runs through the ENA Portal API (ENA
mirrors SRA and DDBJ; GEO accessions are mapped to their SRA study or
experiment first), downloads over HTTPS with resume and MD5 verification, and
needs nothing beyond Python 3.9.

Vendor it into the project so the download stays reproducible:

```sh
mkdir -p scripts && orx skill fastq/assets/fetch_fastq.py > scripts/fetch_fastq.py
```

## Always list first

```sh
python3 scripts/fetch_fastq.py PRJNA229998 --list
```

`--list` prints every run with its layout, library strategy, file count, size,
and sample title, then the total. Read it before downloading anything:

- **Confirm the data matches the question**: organism, `RNA-Seq` vs `WXS`
  (exome) vs `WGS`, paired vs single, and which samples are cases and controls.
  Titles and aliases usually carry the condition; `runinfo.tsv` keeps all
  metadata after the download.
- **Tell the user the total size and where it will go, and wait for their
  go-ahead when it is large.** The script refuses more than 20 GB without
  `--yes` and always refuses a download that would not fit on the disk. Never
  add `--yes` on the user's behalf for a download they have not agreed to.
- Large studies: offer a subset first (`--runs`, `--match`, `--strategy`,
  `--max-runs`) to validate the pipeline before fetching everything.

## Download

```sh
python3 scripts/fetch_fastq.py PRJNA229998 --out data/fastq --parallel 4
python3 scripts/fetch_fastq.py GSE52778 --out data/fastq --match untreated
python3 scripts/fetch_fastq.py PRJEB12345 --runs ERR111,ERR112 --out data/fastq
python3 scripts/fetch_fastq.py PRJNA123456 --strategy WXS --out data/fastq --yes
```

| Flag | Use |
| --- | --- |
| `--runs SRR1,SRR2` | Keep exactly these runs |
| `--match REGEX` | Keep runs whose sample alias/title or experiment title matches |
| `--strategy` | Keep one library strategy: `RNA-Seq`, `WXS`, `WGS`, `AMPLICON`, … |
| `--max-runs N` | Keep the first N runs (pilot runs) |
| `--parallel N` | Concurrent downloads (default 3; ENA tolerates 4–6) |
| `--yes` | Confirm a download above 20 GB the user has agreed to |
| `--sra-fallback` | Convert runs ENA has no FASTQ for with `fasterq-dump` (needs sra-tools) |

Downloads are long: run them in the background or as a detached command and
check progress, rather than blocking a turn. Re-running the same command
resumes partial files and skips files whose MD5 already matches.

## What you get

| File | Contents |
| --- | --- |
| `<run>_1.fastq.gz`, `<run>_2.fastq.gz` | Paired-end reads — the naming ExomeFlow and most pipelines expect |
| `<run>.fastq.gz` | Single-end reads |
| `runinfo.tsv` | Study, sample, layout, strategy, platform, read and base counts, URLs, MD5s, and per-file download status |
| `samplesheet.csv` | `sample,fastq_1,fastq_2,strandedness` with absolute paths — nf-core/rnaseq format |

The script exits non-zero and names the runs that failed; `runinfo.tsv` shows
why. Rerun to retry. A run with no ENA FASTQ (rare, very recent submissions)
needs `--sra-fallback`; ask before installing sra-tools.

## Hand-off

- Exome/genome data → `orx-variant-calling` (ExomeFlow reads `data/fastq`
  directly).
- RNA-seq data → `orx-rnaseq` (use `samplesheet.csv`; join conditions from
  `runinfo.tsv` titles or the paper's supplementary metadata).
- Record the accession, the run list, and the date in the experiment notes;
  public data can be re-processed or withdrawn upstream.

## Rules

- Public archives only. Never upload, mirror, or share controlled-access or
  patient data (dbGaP, EGA, hospital data) with this script or any cloud
  service; controlled-access data needs the user's approved access route.
- Keep FASTQ out of git: put `data/` in `.gitignore`.
- Cite the original study (the BioProject's linked publication) in any output
  that uses the data.
