---
name: orx-variant-calling
description: "Call germline (and tumor-only somatic) variants from exome or genome FASTQ with the PG Lab's ExomeFlow pipeline: references, capture kits, trios and cohorts, CNV, and QC gates before interpretation. Use when the user has WES/WGS/exome reads, asks for variant calling, a VCF, GATK, joint genotyping, or a trio analysis."
---

# Germline variant calling with ExomeFlow

ExomeFlow (`pip install exomeflow`, PG Lab, AIIMS New Delhi) implements GATK
best practices: fastp → BWA-MEM → duplicate marking → BQSR → HaplotypeCaller →
hard filters → ANNOVAR annotation with HPO terms and InterVar ACMG. Prefer it
over hand-built pipelines; fall back to manual GATK steps only for something it
does not cover, and say why.

Fetch public reads with `orx-fastq` — its `<run>_1.fastq.gz`/`<run>_2.fastq.gz`
naming is exactly ExomeFlow's input convention. Interpret results with
`orx-exome-analysis`.

## Before the first run

- **Disk**: references and annotation databases take ~110 GB, plus roughly
  5–10 GB per exome for BAMs and VCFs. Check `df -h` and tell the user.
- **ANNOVAR** cannot be redistributed: the user must register at
  annovar.openbioinformatics.org. ExomeFlow finds an existing install or asks.
- **Capture kit BED**: required for `--joint-genotyping` and `--cnv`, and it
  makes coverage QC meaningful. Ask which kit was used (Agilent SureSelect,
  Twist, Illumina, IDT xGen); for public data, check the paper's methods.
- **Build**: hg38 unless the user's other data is GRCh37 (`--genome-build
  GRCh37`, which annotates with gnomAD v2.1.1).
- First run provisions everything and saves `~/.exomeflow/config.json`; use
  `exomeflow setup` to provision separately. Add `--yes` only for unattended
  runs the user has approved.

## Run

```sh
exomeflow run --input-dir data/fastq --output results/exomeflow \
  --intervals refs/<kit>.hg38.bed --threads 32 --max-workers 2 --yes
```

| Case | Add |
| --- | --- |
| Trio, family, or cohort | `--joint-genotyping --intervals <kit>.bed` (one cohort VCF; required for de novo and segregation analysis) |
| Copy-number variants | `--cnv --intervals <kit>.bed` (read-depth; confirm calls orthogonally) |
| Tumor-only somatic | `--mode somatic` (Mutect2 tumor-only; matched normal is not supported — say so) |
| Existing references | `--reference --dbsnp --mills --known-indels --annovar-bin --annovar-db` |

A 30–100× exome takes hours per sample: launch it as an experiment run on
suitable compute (`orx-compute`) and wait or set a wake-up — never block a turn.
`--max-workers` processes samples in parallel; size it against RAM
(~16–24 GB per worker for GATK).

## QC gates — check before any interpretation

Read `multiqc/exomeflow_report.html`, `Mapsam/<sample>_flagstat.txt`, and the
fastp reports. Compute target coverage with `mosdepth --by <kit>.bed`.

| Metric | Expect (exome) | If not |
| --- | --- | --- |
| Mapped reads | > 95% | Wrong species/build or contamination |
| Duplicate rate | < 20% | Low library complexity; coverage overstated |
| Mean target depth | ≥ 50–100× | Under-calling; state the limit |
| Targets ≥ 20× | > 90% | Name genes of interest with poor coverage |
| Ti/Tv (coding SNVs) | ~2.8–3.0 | Far below: false positives |
| Het/hom-alt ratio | ~1.5–2 (ancestry dependent) | Contamination or consanguinity |
| Sex check (chrX het, chrY coverage) | Matches recorded sex | Sample swap — stop and report |
| Relatedness (trios: `somalier relate`) | Parent–child kinship ~0.25 | Non-paternity or swap — stop and report |
| Contamination (VerifyBamID2) | FREEMIX < 0.03 | Exclude or flag |

A failed sex or relatedness check means the sample identity is in doubt:
report it to the user before any variant is interpreted.

## Outputs

| File | Use |
| --- | --- |
| `Mapsam/<sample>_recalibrated.bam` | IGV review of candidate variants |
| `VCF/<sample>_PASS.vcf` | Filtered calls |
| `VCF/<sample>.annovar.<build>_multianno.txt` | Input to VariantFlow (`orx-exome-analysis`) |
| `VCF/<sample>.annovar.hpo.txt` | Annotations with HPO terms and ACMG classes |
| `VCF/cohort/cohort*.{vcf.gz,txt}` | Joint-genotyped cohort set |
| `logs/` | Full logs — check when `HPO_terms`/`ACMG_classification` columns are missing |

## Alternatives, when justified

- **DeepVariant** (GPU-capable) for higher SNV/indel accuracy on WGS or when
  benchmarking against GIAB; joint-genotype with GLnexus.
- **nf-core/sarek** for tumor–normal somatic pairs, which ExomeFlow does not
  support.
- Benchmark any new pipeline on GIAB HG002 with `hap.py` before trusting it.

## Rules

- Patient sequencing data stays on local or lab-controlled compute. Never send
  reads, BAMs, or VCFs to cloud services or third-party APIs, and never paste
  genotypes into a cloud model's context.
- Use pseudonymous sample IDs in every file name, table, and figure.
- Record tool versions, reference build, capture kit, and resource versions in
  the experiment notes and methods.
- Research results only: clinically actionable findings need confirmation in a
  certified laboratory (e.g. Sanger) before reaching a patient record.
