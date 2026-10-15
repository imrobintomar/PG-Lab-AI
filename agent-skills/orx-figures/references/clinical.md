# Clinical figures: outcomes, effects, cohorts

**Answers:** how outcomes differ between groups over time, how large an
effect is and how certain, how patients flowed through a study, and how a
measurement or predictor performs — the figures of a clinical or surgical
outcomes manuscript.

All numbers come from the analysis dataset and the fitted models; never type
an n, a p-value, or a hazard ratio into a plotting script.

## Journal sizes and files

Build at the printed width and write what the journal accepts. `save()`
writes rasters at 600 dpi, so `formats=("pdf", "svg", "tiff")` covers journals
that require TIFF.

| Journal family | Single column | 1.5 column | Full width |
| --- | --- | --- | --- |
| Nature / Springer Nature | 89 mm (3.5 in) | 120–136 mm | 183 mm (7.2 in) |
| Elsevier (J Pediatr Surg, J Pediatr Urol, J Surg Res) | 90 mm (3.54 in) | 140 mm (5.51 in) | 190 mm (7.48 in) |
| Wiley / BJU Int | 80–85 mm | — | 170–175 mm |
| JAMA / AMA journals | 3.25–3.5 in | — | 6.5–7 in |

Check the author guidelines of the target journal and use its numbers when
they differ; pass the width to `figure(width=…)` or `figure_grid(width=…)`.
The style module exports these as `NATURE_SINGLE`, `NATURE_DOUBLE`,
`ELSEVIER_SINGLE`, `ELSEVIER_ONE_HALF`, and `ELSEVIER_DOUBLE`, and its audit
accepts them.
Line art must stay vector (PDF/EPS) or ≥ 1000 dpi; photographs ≥ 300 dpi.

## Choose the figure by the question

| The figure answers | Use |
| --- | --- |
| Time to an event (survival, recurrence, reoperation) | **Kaplan–Meier** with numbers at risk, log-rank p, Cox HR |
| Effect sizes across subgroups, outcomes, or studies | **Forest plot** with CIs on a log axis for ratios |
| How patients moved through a trial or cohort | **CONSORT** (trials) / **STROBE** (observational) flow diagram |
| A continuous outcome between groups | **Box or violin with every point**, n per group, the test named |
| How well a score or model discriminates | **ROC** with AUC and CI; add calibration for prediction models |
| Agreement between two measurement methods | **Bland–Altman**: difference vs mean, bias and limits of agreement |

## Kaplan–Meier (complete template)

```python
# /// script
# dependencies = ["matplotlib", "numpy", "pandas", "lifelines"]
# ///
import pandas as pd
from lifelines import CoxPHFitter, KaplanMeierFitter
from lifelines.statistics import logrank_test
from orx_figstyle import ELSEVIER_SINGLE, PALETTE, figure, save, use_style

use_style()
df = pd.read_csv("data/analysis/outcomes.csv")          # time_months, event (1/0), group
groups = [("Technique A", PALETTE["blue"]), ("Technique B", PALETTE["orange"])]

fig, ax = figure(width=ELSEVIER_SINGLE, ratio=0.95)      # 90 mm single column
for name, color in groups:
    g = df[df.group == name]
    KaplanMeierFitter(label=f"{name} (n={len(g)})").fit(g.time_months, g.event).plot_survival_function(
        ax=ax, ci_show=True, color=color, show_censors=True, censor_styles={"ms": 3, "marker": "|"})
a, b = (df[df.group == n] for n, _ in groups)
p = logrank_test(a.time_months, b.time_months, a.event, b.event).p_value
cox = CoxPHFitter().fit(df.assign(b=(df.group == groups[1][0]).astype(int))[["time_months", "event", "b"]],
                        "time_months", "event")
hr, lo, hi = cox.summary.loc["b", ["exp(coef)", "exp(coef) lower 95%", "exp(coef) upper 95%"]]
p_text = "< 0.001" if p < 0.001 else f"= {p:.3f}"
ax.text(0.98, 0.97, f"HR {hr:.2f} (95% CI {lo:.2f}–{hi:.2f})\nLog-rank p {p_text}",
        transform=ax.transAxes, ha="right", va="top", fontsize=6)
ax.set_xlabel("Time since surgery (months)")
ax.set_ylabel("Complication-free probability")
ax.set_ylim(0, 1.02)
ax.legend(frameon=False, loc="lower left")

# Numbers at risk, aligned with the x ticks, under the axis.
ticks = [t for t in ax.get_xticks() if 0 <= t <= df.time_months.max()]
ax.set_xticks(ticks)
ax.text(0, -0.30, "At risk", transform=ax.transAxes, fontsize=6, fontweight="bold")
for row, (name, color) in enumerate(groups):
    g = df[df.group == name]
    y = -0.38 - 0.08 * row
    for t in ticks:
        ax.text(t, y, str(int((g.time_months >= t).sum())), transform=ax.get_xaxis_transform(),
                ha="center", fontsize=6, color=color)
save(fig, "figs/km_complication_free", formats=("pdf", "svg", "tiff"))
```

- **Numbers at risk under the x-axis are required** by most surgical journals
  and by every careful reviewer; truncate the x-axis where fewer than ~10% of
  patients remain at risk, and say so.
- Show censoring ticks; state median follow-up in the caption.
- Report the HR from the model you actually fitted (adjusted or not — say
  which), not just the log-rank p.
- p-values: exact to three decimals, "< 0.001" below that; never "p = 0.000".
- The at-risk rows are text under the axis, which is where overlaps appear:
  read `save()`'s audit and adjust the row offsets until it is clean (the
  style module's layout engine makes room for them).

## Patterns for the rest

- **Forest plot**: one row per subgroup or outcome with its n (or events/n),
  point = estimate, horizontal line = 95% CI, vertical reference line at 1
  (ratios, log x-axis with ticks at 0.25, 0.5, 1, 2, 4) or 0 (differences).
  Print the estimate and CI as text in a right-hand column; sort by a stated
  rule; mark the overall estimate with a diamond. Label the favored direction
  under the axis ("Favors A ← → Favors B").
- **CONSORT / STROBE flow**: boxes for assessed, excluded (with reasons and
  counts), allocated/included, lost to follow-up, and analyzed. Every count
  comes from the dataset and the numbers must add up box to box — check the
  arithmetic in the script. Build it with the diagram reference
  (`diagram.md`, TikZ or the HTML diagram template).
- **Group comparison**: box (median, IQR) or violin plus jittered points for
  every patient; n under each group label; name the test (Mann–Whitney,
  t-test, Kruskal–Wallis) and report the effect size with CI. Never use
  bar-plus-error-bar ("dynamite") plots for continuous data.
- **ROC**: diagonal reference, equal aspect, AUC with bootstrap 95% CI in the
  legend; for a prediction model add a calibration plot (observed vs
  predicted, with a histogram of predictions) — discrimination alone is not
  validation. State whether it is the development or a validation cohort.
- **Bland–Altman**: mean bias line and ±1.96 SD limits with their values
  labelled; check and mention proportional bias.

## Rules specific to clinical figures

- No patient identifiers, dates of birth, or photographs that identify a
  patient without documented consent; crop and de-identify clinical images.
- Report denominators everywhere: n per group, events/n, missing data.
- Match the reporting guideline the paper follows (CONSORT, STROBE, STARD,
  TRIPOD) and its required figures.
- Effect sizes with confidence intervals carry the result; p-values alone do
  not.
