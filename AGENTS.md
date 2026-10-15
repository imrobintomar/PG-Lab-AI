# Repository Guide

## What this repository is

PG Lab AI (`imrobintomar/PG-Lab-AI`) is a fork of alphaXiv's OpenResearch. This repository is the Rust implementation of the `orx` command-line tool and the PG Lab AI desktop app. It owns the local CLI, dashboard and API, SQLite store, coding-agent integrations, experiment orchestration, and execution backends. Research projects, experiments, runs, logs, and artifacts stay local to `orx`.

PG Lab AI does not use the `openresearch.sh` service. Sign-in (`orx login`, `logout`, `orgs`, `ssh-key`), managed compute (the `openresearch` backend, `orx instance`, the compute catalog), and `orx feedback` are hidden and refused (see `requires_openresearch_account` in `src/main.rs`), and telemetry stays off because no analytics server is configured (`telemetry_host` in `src/telemetry.rs`). The code for those features is kept but unreachable. Don't re-enable them unless PG Lab AI has its own backend.

## Development guidelines

- Rust code lives in `src/`; the dashboard lives in `ui/src/`. Keep behavior local; don't add calls to `openresearch.sh`.
- Run local app instances through `scripts/dev-slot.mjs` so development data, ports, and processes stay isolated.
- `ui/dist` is committed and embedded in release builds. After UI changes, run `pnpm build` in `ui/` and include the regenerated assets.
- Prefer canonical Tailwind utilities (`flex flex-col h-full min-h-0`) and project theme aliases (`bg-background`, `text-subtext`, `border-border`). Use arbitrary values only when no project utility exists, and preserve semantic marker classes when selectors or runtime behavior depend on them.
- Before shipping, follow the checks in `.github/workflows/ci.yml`.

## CI and release gates

- GitHub protection for `main` must require the `fmt, clippy, test`, `version sanity`, and `linked issue` checks from GitHub Actions, including for administrators. Do not require a merge queue or require branches to be up to date. These settings are managed in GitHub, not by this file.
- `linked issue` fails PRs from forks unless the description links an issue in this repository (e.g. `Closes #N`). PRs from branches in this repository are exempt, since only people with write access can push them. It runs on `pull_request_target`, so it must never check out or run PR code.
- PR CI must test GitHub's simulated merge (`refs/pull/<number>/merge`), which `actions/checkout` selects by default for `pull_request` events, rather than checking out the PR head alone. Each run tests its merge candidate; subsequent changes to `main` do not automatically rerun open PRs.
- CI also runs on `main`. Releases call the same CI workflow on the commit being packaged; publishing requires that run to succeed. Keep `./ci` in cargo-dist's `global-artifacts-jobs` when regenerating the release workflow; `allow-dirty` makes `dist generate` skip it, so follow the steps in `macos/DISTRIBUTION.md` ("CLI binaries").
