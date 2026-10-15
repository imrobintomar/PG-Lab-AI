#!/usr/bin/env python3
"""Search Genomics England PanelApp and export a panel's genes.

The PanelApp API has no text search, so this pages through every panel and
matches the query against panel names and relevant disorders.

  python3 panelapp.py search hypospadias "sex development" hirschsprung
  python3 panelapp.py genes 9 --out panels/dsd.tsv            # green (confidence 3) genes
  python3 panelapp.py genes 234 --min-confidence 2 --out panels/cakut.tsv

Standard library only (Python 3.9+).
"""

from __future__ import annotations

import argparse
import csv
import json
import re
import sys
import urllib.request

API = "https://panelapp.genomicsengland.co.uk/api/v1"
USER_AGENT = "pg-lab-ai-panelapp/1.0 (+https://github.com/imrobintomar/PG-Lab-AI)"


def get(url: str) -> dict:
    req = urllib.request.Request(url, headers={"User-Agent": USER_AGENT, "Accept": "application/json"})
    with urllib.request.urlopen(req, timeout=60) as resp:
        return json.load(resp)


def pages(url: str):
    while url:
        data = get(url)
        yield from data.get("results", [])
        url = data.get("next")


def search(terms: list[str]) -> None:
    pattern = re.compile("|".join(re.escape(t) for t in terms), re.IGNORECASE)
    hits = 0
    for panel in pages(f"{API}/panels/?page_size=100"):
        disorders = panel.get("relevant_disorders") or []
        text = " ".join([panel.get("name", ""), *disorders])
        if pattern.search(text):
            hits += 1
            stats = panel.get("stats") or {}
            matched = [d for d in disorders if pattern.search(d)][:3]
            print(f"{panel['id']:>6}  v{panel.get('version', '?'):<8} {panel.get('name', '')}"
                  f"  [{stats.get('number_of_genes', '?')} genes]"
                  + (f"  — {'; '.join(matched)}" if matched else ""))
    if not hits:
        print("No panel matched. Try a broader term (e.g. 'kidney', 'sex development').", file=sys.stderr)
        sys.exit(1)


def genes(panel_id: int, min_confidence: int, out: str | None) -> None:
    panel = get(f"{API}/panels/{panel_id}/")
    rows = []
    for gene in panel.get("genes", []):
        confidence = int(gene.get("confidence_level") or 0)
        if confidence < min_confidence:
            continue
        data = gene.get("gene_data") or {}
        rows.append({
            "gene_symbol": gene.get("entity_name") or data.get("gene_symbol", ""),
            "hgnc_id": data.get("hgnc_id", ""),
            "confidence_level": confidence,
            "mode_of_inheritance": gene.get("mode_of_inheritance", ""),
            "phenotypes": "; ".join(gene.get("phenotypes") or []),
            "panel": panel.get("name", ""),
            "panel_version": panel.get("version", ""),
        })
    rows.sort(key=lambda r: r["gene_symbol"])
    handle = open(out, "w", newline="") if out else sys.stdout
    writer = csv.DictWriter(handle, fieldnames=list(rows[0]) if rows else ["gene_symbol"], delimiter="\t")
    writer.writeheader()
    writer.writerows(rows)
    if out:
        handle.close()
    print(f"{panel.get('name')} v{panel.get('version')}: {len(rows)} genes at confidence ≥ {min_confidence}"
          + (f" → {out}" if out else ""), file=sys.stderr)


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest="cmd", required=True)
    s = sub.add_parser("search", help="find panels by name or relevant disorder")
    s.add_argument("terms", nargs="+")
    g = sub.add_parser("genes", help="export a panel's genes as TSV")
    g.add_argument("panel_id", type=int)
    g.add_argument("--min-confidence", type=int, default=3, choices=[1, 2, 3],
                   help="3 = green (diagnostic grade, default), 2 = amber, 1 = red")
    g.add_argument("--out")
    args = ap.parse_args()
    if args.cmd == "search":
        search(args.terms)
    else:
        genes(args.panel_id, args.min_confidence, args.out)


if __name__ == "__main__":
    main()
