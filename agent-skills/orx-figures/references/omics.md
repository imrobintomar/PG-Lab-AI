# Omics figures: RNA-seq, enrichment, variants

**Answers:** what changed in expression, which pathways move together, how
samples group, and where variants fall — the figures of a transcriptomics or
genomics manuscript.

Every value comes from a results table (DESeq2 output, enrichment table,
annotated variants); load the file, never retype numbers. Build at the
journal's printed width (see *Journal sizes* in `clinical.md`).

## Choose the figure by the question

| The figure answers | Use |
| --- | --- |
| Which genes changed, by how much, how confidently | **Volcano**: shrunken log2FC vs −log10 padj |
| Is the fold change dependent on expression level | **MA plot**: mean normalized count (log) vs log2FC |
| Do samples group by condition, or by batch | **PCA** of VST counts, colored by condition, shaped by batch |
| How do the top genes behave across samples | **Heatmap** of row z-scored VST values, clustered, with a condition annotation bar |
| Which pathways are enriched, in which direction | **Enrichment dot plot**: NES (or gene ratio) vs pathway, size = genes, color = padj |
| Association across the genome | **Manhattan** with genome-wide line, plus **QQ** to show calibration |
| Where variants fall on a protein | **Lollipop** over the protein with domains |
| What kinds of variants a cohort carries | **Stacked bars** of consequence per sample, or a gene × sample tile (oncoprint-style) |

## Volcano (complete template)

```python
# /// script
# dependencies = ["matplotlib", "numpy", "pandas", "adjustText"]
# ///
import numpy as np, pandas as pd
from adjustText import adjust_text
from orx_figstyle import MUTED, NATURE_SINGLE, PALETTE, figure, save, use_style

use_style()
de = pd.read_csv("results/de/case_vs_control_all_genes.tsv", sep="\t").dropna(subset=["padj"])
de["nlp"] = -np.log10(de["padj"].clip(lower=1e-300))
up = (de.padj < 0.05) & (de.log2FoldChange >= 1)
down = (de.padj < 0.05) & (de.log2FoldChange <= -1)

fig, ax = figure(width=NATURE_SINGLE, ratio=0.9)     # 89 mm single column
ax.scatter(de.loc[~(up | down), "log2FoldChange"], de.loc[~(up | down), "nlp"],
           s=3, color=MUTED, rasterized=True, linewidths=0)
ax.scatter(de.loc[up, "log2FoldChange"], de.loc[up, "nlp"], s=4, color=PALETTE["red"],
           linewidths=0, rasterized=True, label=f"Up ({up.sum()})")
ax.scatter(de.loc[down, "log2FoldChange"], de.loc[down, "nlp"], s=4, color=PALETTE["blue"],
           linewidths=0, rasterized=True, label=f"Down ({down.sum()})")
for x in (-1, 1):
    ax.axvline(x, color="0.6", lw=0.5, ls="--")
ax.axhline(-np.log10(0.05), color="0.6", lw=0.5, ls="--")
top = de[up | down].nsmallest(10, "padj")              # label the strongest few only
labels = [ax.text(r.log2FoldChange, r.nlp, r.symbol, fontsize=5, style="italic")
          for r in top.itertuples()]
adjust_text(labels, ax=ax, arrowprops=dict(arrowstyle="-", color="0.4", lw=0.4))
ax.set_xlabel("log$_2$ fold change (case vs control)")
ax.set_ylabel("−log$_{10}$ adjusted p")
ax.legend(frameon=False, loc="lower center", bbox_to_anchor=(0.5, 1.0), ncol=2)
save(fig, "figs/volcano", formats=("pdf", "svg", "tiff"))
```

- **Rasterize the point clouds, not the axes** (`rasterized=True` on the
  scatter): 20,000 vector dots make a PDF that crashes viewers, while axes and
  text stay vector.
- Thresholds drawn on the plot must equal the thresholds in the methods.
- Label at most ~10–15 genes, chosen by a stated rule (lowest padj);
  `adjust_text` moves labels off each other and off the points. Re-read the
  audit — overlapping labels are a defect.
- The caption states the contrast, the test, padj and |log2FC| thresholds,
  shrinkage method, and n per group.

## Patterns for the rest

- **MA plot**: `ax.set_xscale("log")` on baseMean; same colors as the volcano
  so the two read together.
- **PCA**: compute on VST/rlog of the top 500 most variable genes; axis labels
  carry the variance explained (`PC1 (38%)`). Color = condition, marker =
  batch; equal aspect when PC scales are comparable. One point per sample,
  never a fitted ellipse without saying what it represents.
- **Heatmap**: z-score each gene across samples, `cmap=DIVERGING` with
  symmetric limits, cluster rows (and columns only if the order is not the
  design), add a thin condition bar above the columns, and a colorbar labelled
  "Row z-score". More than ~60 genes stop being readable — show the top genes
  and put the full list in a supplement. See `matrix.md` for colormap rules.
- **Enrichment dot plot**: one row per pathway sorted by NES, x = NES (or gene
  ratio), marker size = gene count with a size legend, color = −log10 padj
  using `SEQUENTIAL`. Shorten pathway names consistently; state the database
  and version in the caption.
- **Manhattan**: alternate two greys by chromosome, genome-wide line at
  5×10⁻⁸ (or the burden test's Bonferroni threshold), highlight and label only
  significant loci. Pair with a QQ plot reporting λ (genomic inflation).
  Rasterize points.
- **Lollipop**: x = amino-acid position, stems with heads sized by recurrence
  and colored by consequence (missense, truncating, splice, in-frame); draw
  the protein as a bar with domain boxes from UniProt/Pfam; name the
  transcript (MANE Select) in the caption.
- **Variant summary**: stacked bars per sample (or per gene class) of
  consequence counts, a consistent category order, counts not percentages
  unless denominators differ — and then say so.

## Rules specific to omics

- Gene symbols in italics in the caption and text (*SRY*); proteins upright.
- Report n per group and replicate type (biological, not technical) in every
  caption.
- Never show patient identifiers; use pseudonymous sample IDs.
- Put full tables (all genes, all pathways) in the supplement; the figure
  shows the finding.
