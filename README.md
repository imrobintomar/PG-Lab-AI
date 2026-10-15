<div align="center">

<h1><img src=".github/readme-assets/pg-lab-ai.png" alt="" width="56" align="absmiddle" /> PG Lab AI</h1>

An AI-powered workspace for scientific discovery.

<p>
<a href="https://github.com/imrobintomar/PG-Lab-AI/releases/latest"><img src="https://img.shields.io/github/v/release/imrobintomar/PG-Lab-AI?style=flat-square" alt="Latest release" /></a>
<a href="https://github.com/imrobintomar/PG-Lab-AI/blob/main/LICENSE"><img src="https://img.shields.io/github/license/imrobintomar/PG-Lab-AI?style=flat-square" alt="License" /></a>
</p>

<p><a href="README.md">English</a> · <a href="README.es.md">Español</a> · <a href="README.ko.md">한국어</a> · <a href="README.zh.md">简体中文</a> · <a href="README.ja.md">日本語</a></p>

<p><img src=".github/readme-assets/pg-lab-ai-screenshot.png" alt="PG Lab AI desktop app running a research agent on a local Qwen model through Ollama" width="680" /></p>

<hr />

<p>Whether you’re looking for a copilot or an autoresearch tool, PG Lab AI will accelerate your work. Launch research agents that can review literature, develop hypotheses, run experiments, and produce research artifacts.</p>

<p>
<a href="https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/PG-Lab-AI.dmg"><picture><source media="(prefers-color-scheme: dark)" srcset=".github/readme-assets/download-macos-dark.svg"><img src=".github/readme-assets/download-macos.svg" alt="Download PG Lab AI for macOS" width="220" height="44" /></picture></a>
<a href="https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/PG-Lab-AI-Setup.exe"><picture><source media="(prefers-color-scheme: dark)" srcset=".github/readme-assets/download-windows-dark.svg"><img src=".github/readme-assets/download-windows.svg" alt="Download PG Lab AI for Windows (Beta)" width="220" height="44" /></picture></a>
<a href="https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/PG-Lab-AI-x86_64.AppImage"><picture><source media="(prefers-color-scheme: dark)" srcset=".github/readme-assets/download-linux-dark.svg"><img src=".github/readme-assets/download-linux.svg" alt="Download PG Lab AI for Linux" width="220" height="44" /></picture></a>
</p>

<p>
<a href="https://github.com/imrobintomar/PG-Lab-AI/tree/main/docs"><img src=".github/readme-assets/action-documentation.svg" alt="Documentation" width="132" height="24" /></a><img src=".github/readme-assets/action-separator.svg" alt=" · " width="12" height="24" />
<a href="https://github.com/imrobintomar/PG-Lab-AI/releases"><img src=".github/readme-assets/action-releases.svg" alt="Releases" width="78" height="24" /></a>
</p>

<p><sub>macOS 11+ · Windows beta requires <a href="docs/windows.md">Git for Windows</a> · <a href="docs/linux.md">Linux</a> app needs glibc 2.35+</sub></p>

</div>

## Get started

**We recommend using the PG Lab AI desktop app.** Download it for your platform:

- [macOS](https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/PG-Lab-AI.dmg)
- [Windows (Beta)](https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/PG-Lab-AI-Setup.exe)
- Linux: [x86_64](https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/PG-Lab-AI-x86_64.AppImage) · [ARM64](https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/PG-Lab-AI-aarch64.AppImage)

<details>
<summary>Prefer the standalone CLI? Install on macOS or Linux</summary>

```sh
curl -LsSf https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/pg-lab-ai-cli-installer.sh | sh
orx up
```

`orx up` opens the local dashboard at `http://127.0.0.1:4791`.

You can also install `orx` for your terminal from the app's
Settings → Updates → **Install the `orx` command**.

</details>

<details>
<summary>First launch: the apps are not yet code-signed</summary>

- **macOS:** right-click PG Lab AI in Applications and choose **Open**, then **Open** again.
- **Windows:** on the SmartScreen warning, choose **More info** → **Run anyway**.
- **Linux:** make the AppImage executable (`chmod +x PG-Lab-AI-*.AppImage`) and run it.

On a managed Mac, device-management policies may block unsigned apps entirely.

</details>

## How it works

**Use your favorite coding agent.** PG Lab AI works natively with <img src=".github/readme-assets/claude.svg" alt="" width="18" height="18" align="texttop" /> Claude Code,
<img src=".github/readme-assets/codex.svg" alt="" width="18" height="18" align="texttop" /> Codex,
<img src=".github/readme-assets/opencode.svg" alt="" width="18" height="18" align="texttop" /> OpenCode,
<img src=".github/readme-assets/cursor.svg" alt="" width="18" height="18" align="texttop" /> Cursor, and
<img src=".github/readme-assets/antigravity.svg" alt="" width="18" height="18" align="texttop" /> Google Antigravity.
You can also [use a local model](docs/local-models.md) with OpenCode through
<img src=".github/readme-assets/lmstudio.svg" alt="" width="18" height="18" align="texttop" /> LM Studio,
<img src=".github/readme-assets/omlx.svg" alt="" width="18" height="18" align="texttop" /> oMLX,
<img src=".github/readme-assets/ollama.svg" alt="" width="18" height="18" align="texttop" /> Ollama, or a custom endpoint.
**Local by default:** when Ollama is running with `qwen3.8-cc:27b-q4_K_M`, PG Lab AI
connects it automatically and uses it for new chats, so no cloud account is needed.

**Ground research hypotheses in the latest literature.** PG Lab AI integrates with <img src=".github/readme-assets/alphaxiv.svg" alt="" width="18" height="18" align="absmiddle" /> alphaXiv, <img src=".github/readme-assets/biorxiv.svg" alt="" width="18" height="18" align="absmiddle" /> bioRxiv, <img src=".github/readme-assets/pubmed.svg" alt="" width="18" height="18" align="absmiddle" /> PubMed, Europe PMC, and ClinicalTrials.gov so agents can find relevant papers, develop informed hypotheses, and connect their experiments to existing research.

**Choose your compute.** Run experiments locally or use SSH,
Slurm,
<img src=".github/readme-assets/kubernetes.svg" alt="" width="18" height="18" align="texttop" /> Kubernetes,
<img src=".github/readme-assets/modal.svg" alt="" width="18" height="18" align="texttop" /> Modal,
<img src=".github/readme-assets/thinking-machines.svg" alt="" width="18" height="18" align="texttop" /> Tinker,
<img src=".github/readme-assets/ray.svg" alt="" width="18" height="18" align="texttop" /> Ray, and
<img src=".github/readme-assets/huggingface.svg" alt="" width="18" height="18" align="texttop" /> Hugging Face Jobs.

**Give agents a memory of every experiment.** PG Lab AI records every experiment in a local SQL database and keeps its logs, code,
and artifacts on your machine so that agents can
learn from past results and decide what to try next. The desktop app makes it
easy to see what your agents have done and inspect the evidence behind each result.

**Keep and own your research.** Run entirely locally. We don't collect your code or agent traces. Your projects,
conversations, experiments, logs, and artifacts stay on your machine, under your
control.

## Usage analytics

PG Lab AI does not collect usage analytics and has no server of its own. Only
the services you choose to use receive data: your coding agent's model provider,
and the literature sources you search (alphaXiv, PubMed, OpenAlex, bioRxiv).
