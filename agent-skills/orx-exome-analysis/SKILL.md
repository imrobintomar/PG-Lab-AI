---
name: orx-exome-analysis
description: "Interpret annotated exome/genome variants downstream of calling: VariantFlow filtering, ACMG review, phenotype (HPO) and gene-panel prioritization, trio/family inheritance, consanguinity, cohort burden, and candidate reports. Use when the user has ANNOVAR/ExomeFlow output or a VCF and asks for candidate variants, a diagnosis, segregation, gene burden, or exome results."
---

# Downstream exome analysis

Input is ExomeFlow output (`orx-variant-calling`):
`VCF/<sample>.annovar.<build>_multianno.txt` and
`VCF/<sample>.annovar.hpo.txt`, or the cohort equivalents. A plain VCF must be
annotated first (ANNOVAR, VEP, or ExomeFlow).

## 1. Collect the clinical question first

Ask for, and record in the experiment notes:

- **Phenotype as HPO terms** (e.g. HP:0000047 Hypospadias,
  HP:0002251 Aganglionic megacolon). Free text is not enough to prioritize.
- **Family structure**: singleton, trio, affected siblings; which members are
  affected. Joint-genotyped trios are needed for de novo calls.
- **Consanguinity**: common in many Indian cohorts; it shifts priority to
  homozygous recessive variants inside runs of homozygosity
  (`bcftools roh`).
- **Ancestry** and **sex**.

## 2. Run VariantFlow (PG Lab)

VariantFlow (github.com/imrobintomar/VariantFlow) reads ANNOVAR multianno
files and does filtering, InterVar ACMG evidence, scoring, gene ranking,
GO/KEGG/Reactome enrichment, and HTML/Excel/PDF reports with a methods file.

```sh
git clone https://github.com/imrobintomar/VariantFlow.git tools/VariantFlow
pip install -e tools/VariantFlow --no-build-isolation
python tools/VariantFlow/variantflow_run.py analyze VCF/P01.annovar.hg38_multianno.txt \
  --output results/variantflow/P01 --sample-id P01 --genome hg38 --af 0.01 --min-dp 10
python tools/VariantFlow/variantflow_run.py family family_dir/ \
  --proband P01 --father F01 --mother M01 --output results/variantflow/family
python tools/VariantFlow/variantflow_run.py cohort cohort_dir/ --output results/variantflow/cohort
```

`family` writes de novo, autosomal recessive, compound heterozygous, and
X-linked tables; `cohort` writes shared/unique variants, gene burden, and
recurrent genes. Read `methods.txt` and `statistics.json`, and check the
filtering funnel: a step that removes almost everything usually means a
missing annotation column.

## 3. Prioritize — the reasoning VariantFlow's tables need

| Layer | Default | Notes |
| --- | --- | --- |
| Quality | PASS, DP ≥ 10, GQ ≥ 20, het allele balance 0.25–0.75 | Relax only with IGV review |
| Population frequency | Dominant ≤ 0.0001 (often absent); recessive ≤ 0.01 | Use gnomAD v4 *grpmax*. South Asians are under-represented — also check South Asian AF and IndiGen/GenomeAsia when available, and an in-house cohort |
| Consequence | LoF (stop, frameshift, canonical splice), missense, in-frame indel, splice region | LoF only matters in LoF-intolerant genes (gnomAD LOEUF) or known LoF mechanism |
| Missense predictors | REVEL ≥ 0.644 PP3-supporting, ≥ 0.773 moderate, ≥ 0.932 strong; ≤ 0.290 BP4 | ClinGen-calibrated thresholds; one predictor, applied once |
| Splicing | SpliceAI ≥ 0.2 supports, ≥ 0.5 strong; ≤ 0.1 no impact | Confirm with RNA when it matters |
| Inheritance | de novo, homozygous, compound het (in trans), X-linked hemizygous | Phase compound hets with parents |
| Phenotype fit | Gene–disease validity and HPO overlap | Exomiser can rank by HPO if installed |

### Gene panels for pediatric surgical conditions

Start from curated panels rather than memory. The bundled helper searches
Genomics England PanelApp (its API has no text search, so the helper pages
through every panel name and relevant disorder) and exports a panel's genes:

```sh
orx skill exome-analysis/assets/panelapp.py > scripts/panelapp.py
python3 scripts/panelapp.py search "sex development" hirschsprung anorectal CAKUT
python3 scripts/panelapp.py genes 9 --out panels/dsd.tsv          # green genes only
```

| Condition | PanelApp panel |
| --- | --- |
| Hypospadias, DSD | Differences in sex development (id 9) |
| Kidney and urinary tract anomalies | CAKUT (id 234) |
| Hirschsprung disease | Familial Hirschsprung Disease (id 63) |
| Anorectal malformations | Non-syndromic familial congenital anorectal malformations (id 253) |

Panel ids are stable but versions change: rerun `search` and record the
version the helper prints. Use green (confidence 3) genes for diagnostic
filtering and amber (`--min-confidence 2`) only for research expansion, and
check ClinGen gene–disease validity. Conditions without a dedicated panel
(biliary atresia, congenital diaphragmatic hernia, oesophageal atresia) need a
literature-derived gene list from `orx-lit-review`, labelled as such. Report panel-negative results as "no qualifying variant in
<panel vN>", not as a negative exome.

## 4. ACMG/AMP classification

InterVar's automated ACMG call is a starting point, not a classification.

- Review each criterion manually for every reported variant, following ClinGen
  SVI guidance: PVS1 decision tree, PM2 at *supporting* strength, calibrated
  PP3/BP4, and de novo (PS2/PM6) only with confirmed parentage.
- In silico evidence alone never makes a variant Pathogenic or Likely
  Pathogenic.
- Check ClinVar review status (stars) and the date; one-star submissions are
  weak.
- Look up the literature with `orx-lit-review` (PubMed, Europe PMC) and cite it.

## 5. Review every reported candidate

- Open the BAM in IGV at the variant; save a batch snapshot
  (`igv.sh -b snapshot.bat`) for the report.
- Confirm segregation in the family and that coverage is adequate in the gene.
- Note what was not tested: low-coverage exons, CNVs, repeats, deep intronic,
  mitochondrial variants.

## 6. Cohort and burden analysis

- Case–control burden needs matched controls of the same ancestry and
  capture kit; gnomAD is not a matched control set — say so if it is used.
- Collapse qualifying variants per gene; test with Fisher's exact test or
  SKAT-O; correct for the number of genes tested (Bonferroni ~2.5×10⁻⁶ for
  exome-wide).
- With small cohorts, report recurrent genes as hypotheses, not associations.

## 7. Deliverables

- `results/candidates.tsv`: gene, HGVS (c. and p.), zygosity, inheritance,
  gnomAD AF, predictors, ClinVar, ACMG criteria and class, evidence, IGV link.
- A short report with `orx-reports`; manuscript figures with `orx-figures` →
  `references/omics.md` (variant consequence summary, lollipop plot,
  pedigree-annotated tables).

## Rules

- Patient data stays local. Use pseudonymous IDs everywhere; never send
  genotypes, VCFs, or reports to cloud services.
- Secondary findings (ACMG SF list) only when the participant consented to
  receive them.
- Research results are not clinical reports: state that clinically relevant
  findings require confirmation in a certified laboratory.
