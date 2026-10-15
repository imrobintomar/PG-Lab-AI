---
name: orx-rnaseq
description: "Analyze bulk RNA-seq from FASTQ to differential expression: design check, QC, Salmon or nf-core/rnaseq quantification, DESeq2, enrichment, and methods text. Use when the user has RNA-seq reads or counts, asks for differential expression, DEGs, pathway or GSEA analysis, or wants to reanalyze a GEO/SRA RNA-seq study."
---

# Bulk RNA-seq

Fetch public reads with `orx-fastq`. Run every long step as an experiment run
(`orx-compute`), keep scripts in the project, and keep `data/` out of git.

## 1. Check the design before computing

Write `metadata.csv` (one row per sample: `sample`, `condition`, plus every
known covariate — batch, sex, age, tissue, sequencing run, patient for paired
designs). Then check, and tell the user what you find:

- **Replicates**: at least 3 biological replicates per group. Fewer gives
  unreliable dispersion estimates; say so instead of over-interpreting.
- **Confounding**: if every case was sequenced in one batch and every control
  in another, batch and condition cannot be separated. Stop and report it.
- **Pairing**: samples from the same patient (pre/post, tumor/normal) need
  `~ patient + condition`.
- **Organism and build**: use one reference for everything — human GENCODE
  GRCh38 (primary assembly FASTA + matching GTF) unless the study used another.

## 2. Choose the pipeline

| Situation | Route |
| --- | --- |
| Nextflow plus Docker/Singularity/conda available, or many samples | **nf-core/rnaseq** — the best-maintained, fully QC'd pipeline |
| Small study, laptop or workstation, no container runtime | **fastp → Salmon → tximport → DESeq2** below |
| Already have a counts matrix | Skip to step 4 |

Check what is installed first (`command -v nextflow salmon fastp hisat2
samtools R docker singularity mamba`). Install missing tools with
`mamba`/`conda` from bioconda into a project environment, with the user's OK.

### nf-core/rnaseq

```sh
nextflow run nf-core/rnaseq -r 3.14.0 -profile docker \
  --input data/fastq/samplesheet.csv --outdir results/rnaseq \
  --fasta GRCh38.primary_assembly.genome.fa.gz --gtf gencode.v46.annotation.gtf.gz \
  --aligner star_salmon
```

`orx-fastq` writes `samplesheet.csv` in this format with `strandedness=auto`.
Pin the release (`-r`) and record it. STAR needs ~32 GB RAM; on less, use
`--aligner hisat2` or the Salmon route.

### fastp → Salmon (lightweight)

```sh
fastp -i S_1.fastq.gz -I S_2.fastq.gz -o trim/S_1.fq.gz -O trim/S_2.fq.gz \
  -j qc/S.fastp.json -h qc/S.fastp.html -w 8
salmon index -t gentrome.fa.gz -d decoys.txt -i salmon_idx -k 31   # once; decoy-aware
salmon quant -i salmon_idx -l A -1 trim/S_1.fq.gz -2 trim/S_2.fq.gz \
  --validateMappings --gcBias --seqBias -p 8 -o quant/S
multiqc qc quant -o qc/multiqc
```

`-l A` detects library type; read it from `quant/S/lib_format_counts.json`
and keep it consistent across samples.

## 3. QC gates — report each, do not silently continue

| Metric | Expect | If not |
| --- | --- | --- |
| Reads per sample | ≥ 10–20 M for DE | Flag low-depth samples |
| Mapping / Salmon mapping rate | > 70% (human, polyA) | Check species, contamination, rRNA |
| Duplication, adapter content (fastp) | Consistent across samples | Investigate outliers |
| Strandedness | Same for all samples | Mixed protocols — model or split |
| PCA / sample distances | Replicates cluster by condition or batch | Name outliers; never drop one without a stated reason |

## 4. Differential expression (DESeq2)

Prefer R/Bioconductor DESeq2 (`tximport` for Salmon). Use `pydeseq2` only when
R is unavailable and say so in the methods.

```r
txi <- tximport(files, type = "salmon", tx2gene = tx2gene)
dds <- DESeqDataSetFromTximport(txi, metadata, design = ~ batch + condition)
dds <- dds[rowSums(counts(dds) >= 10) >= min_group_size, ]
dds$condition <- relevel(dds$condition, ref = "control")
dds <- DESeq(dds)
res <- results(dds, contrast = c("condition", "case", "control"), alpha = 0.05)
shrunk <- lfcShrink(dds, coef = "condition_case_vs_control", type = "apeglm")
vsd <- vst(dds)   # PCA, heatmaps, clustering — never for testing
```

- Significance is `padj < 0.05` (Benjamini–Hochberg). An absolute log2 fold
  change cut-off, if used, must be stated; prefer
  `results(..., lfcThreshold = 1)` over filtering afterwards.
- Report shrunken log2FC for ranking and plotting; test on unshrunken.
- Write the full results table (all genes, with gene symbols, baseMean,
  log2FC, lfcSE, pvalue, padj) plus normalized counts — not just the
  significant subset.
- Never use raw counts, TPM, or FPKM to compare groups; never run t-tests per
  gene.

## 5. Enrichment

- **GSEA** (preferred): rank all tested genes by the Wald statistic; use
  `fgsea`/`clusterProfiler::GSEA` or `gseapy.prerank` with MSigDB Hallmark,
  GO BP, and Reactome.
- **Over-representation**: only significant genes against the universe of
  *tested* genes, never the whole genome.
- Report adjusted p-values and the gene-set database version.

## 6. Deliverables

- `results/de/<contrast>_all_genes.tsv`, `..._significant.tsv`,
  `normalized_counts.tsv`, enrichment tables.
- Figures with `orx-figures` → `references/omics.md`: PCA, volcano, MA,
  top-gene heatmap, enrichment dot plot.
- A methods paragraph with every tool and version, reference and annotation
  release, filters, design formula, and thresholds; save it with
  `orx-reports`.

## Rules

- Patient-derived data stays on local or lab-controlled compute; keep
  identifiers pseudonymized in every output.
- Every number in the text comes from a results file; never estimate a DEG
  count or a fold change.
- State the limitations you found: small n, confounding, outliers, depth.
