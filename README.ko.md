<div align="center">

<h1><img src=".github/readme-assets/pg-lab-ai.png" alt="" width="56" align="absmiddle" /> PG Lab AI</h1>

과학적 발견을 위한 AI 기반 워크스페이스.

<p>
<a href="https://github.com/imrobintomar/PG-Lab-AI/releases/latest"><img src="https://img.shields.io/github/v/release/imrobintomar/PG-Lab-AI?style=flat-square" alt="Latest release" /></a>
<a href="https://github.com/imrobintomar/PG-Lab-AI/blob/main/LICENSE"><img src="https://img.shields.io/github/license/imrobintomar/PG-Lab-AI?style=flat-square" alt="License" /></a>
</p>

<p><a href="README.md">English</a> · <a href="README.es.md">Español</a> · <a href="README.ko.md">한국어</a> · <a href="README.zh.md">简体中文</a> · <a href="README.ja.md">日本語</a></p>

<p><img src=".github/readme-assets/pg-lab-ai-screenshot.png" alt="PG Lab AI desktop app running a research agent on a local Qwen model through Ollama" width="680" /></p>

<hr />

<p>코파일럿이 필요하든 자율 연구 도구가 필요하든, PG Lab AI는 작업을 가속합니다. 문헌을 검토하고, 가설을 세우고, 실험을 실행하고, 연구 산출물을 만드는 연구 에이전트를 시작하세요.</p>

<p>
<a href="https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/PG-Lab-AI.dmg"><picture><source media="(prefers-color-scheme: dark)" srcset=".github/readme-assets/download-macos-dark.svg"><img src=".github/readme-assets/download-macos.svg" alt="Download PG Lab AI for macOS" width="220" height="44" /></picture></a>
<a href="https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/PG-Lab-AI-Setup.exe"><picture><source media="(prefers-color-scheme: dark)" srcset=".github/readme-assets/download-windows-dark.svg"><img src=".github/readme-assets/download-windows.svg" alt="Download PG Lab AI for Windows (베타)" width="220" height="44" /></picture></a>
<a href="https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/PG-Lab-AI-x86_64.AppImage"><picture><source media="(prefers-color-scheme: dark)" srcset=".github/readme-assets/download-linux-dark.svg"><img src=".github/readme-assets/download-linux.svg" alt="Download PG Lab AI for Linux" width="220" height="44" /></picture></a>
</p>

<p>
<a href="https://github.com/imrobintomar/PG-Lab-AI/tree/main/docs"><img src=".github/readme-assets/action-documentation.svg" alt="Documentation" width="132" height="24" /></a><img src=".github/readme-assets/action-separator.svg" alt=" · " width="12" height="24" />
<a href="https://github.com/imrobintomar/PG-Lab-AI/releases"><img src=".github/readme-assets/action-releases.svg" alt="Releases" width="78" height="24" /></a>
</p>

<p><sub>macOS 11+ · Windows 베타는 다음이 필요합니다: <a href="docs/windows.md">Git for Windows</a> · <a href="docs/linux.md">Linux</a> 앱은 glibc 2.35+가 필요합니다</sub></p>

</div>

## 시작하기

**PG Lab AI 데스크톱 앱 사용을 권장합니다.** 사용 중인 플랫폼에 맞게 다운로드하세요:

- [macOS](https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/PG-Lab-AI.dmg)
- [Windows (베타)](https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/PG-Lab-AI-Setup.exe)
- Linux: [x86_64](https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/PG-Lab-AI-x86_64.AppImage) · [ARM64](https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/PG-Lab-AI-aarch64.AppImage)

<details>
<summary>독립 실행형 CLI를 선호하시나요? macOS 또는 Linux에 설치하세요</summary>

```sh
curl -LsSf https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/pg-lab-ai-cli-installer.sh | sh
orx up
```

`orx up`은 `http://127.0.0.1:4791`에서 로컬 대시보드를 엽니다.

앱의 Settings → Updates → **Install the `orx` command**에서 터미널용 `orx`도 설치할 수 있습니다.

</details>

<details>
<summary>처음 실행: 앱은 아직 코드 서명되지 않았습니다</summary>

- **macOS:** 응용 프로그램 폴더에서 PG Lab AI를 마우스 오른쪽 버튼으로 클릭하고 **열기**를 선택한 다음 다시 **열기**를 선택하세요.
- **Windows:** SmartScreen 경고에서 **추가 정보** → **실행**을 선택하세요.
- **Linux:** AppImage에 실행 권한을 주고(`chmod +x PG-Lab-AI-*.AppImage`) 실행하세요.

관리 대상 Mac에서는 기기 관리 정책이 서명되지 않은 앱을 완전히 차단할 수 있습니다.

</details>

## 작동 방식

**원하는 코딩 에이전트를 사용하세요.** PG Lab AI는 다음 도구를 기본 지원합니다: <img src=".github/readme-assets/claude.svg" alt="" width="18" height="18" align="texttop" /> Claude Code,
<img src=".github/readme-assets/codex.svg" alt="" width="18" height="18" align="texttop" /> Codex,
<img src=".github/readme-assets/opencode.svg" alt="" width="18" height="18" align="texttop" /> OpenCode,
<img src=".github/readme-assets/cursor.svg" alt="" width="18" height="18" align="texttop" /> Cursor 및
<img src=".github/readme-assets/antigravity.svg" alt="" width="18" height="18" align="texttop" /> Google Antigravity.
OpenCode에서 다음 도구를 통해 [로컬 모델을 사용할 수도 있습니다](docs/local-models.md):
<img src=".github/readme-assets/lmstudio.svg" alt="" width="18" height="18" align="texttop" /> LM Studio,
<img src=".github/readme-assets/omlx.svg" alt="" width="18" height="18" align="texttop" /> oMLX,
<img src=".github/readme-assets/ollama.svg" alt="" width="18" height="18" align="texttop" /> Ollama 또는 사용자 지정 엔드포인트.
**기본은 로컬:** Ollama에서 `qwen3.8-cc:27b-q4_K_M`이 실행 중이면 PG Lab AI가 자동으로 연결해
새 채팅에 사용합니다. 클라우드 계정이 필요 없습니다.

**최신 문헌을 근거로 연구 가설을 세우세요.** PG Lab AI는 <img src=".github/readme-assets/alphaxiv.svg" alt="" width="18" height="18" align="absmiddle" /> alphaXiv, <img src=".github/readme-assets/biorxiv.svg" alt="" width="18" height="18" align="absmiddle" /> bioRxiv, <img src=".github/readme-assets/pubmed.svg" alt="" width="18" height="18" align="absmiddle" /> PubMed, Europe PMC, ClinicalTrials.gov와 연동되어 에이전트가 관련 논문을 찾고, 근거 있는 가설을 세우고, 실험을 기존 연구와 연결할 수 있습니다.

**컴퓨팅 환경을 선택하세요.** 로컬에서 실험을 실행하거나 SSH,
Slurm,
<img src=".github/readme-assets/kubernetes.svg" alt="" width="18" height="18" align="texttop" /> Kubernetes,
<img src=".github/readme-assets/modal.svg" alt="" width="18" height="18" align="texttop" /> Modal,
<img src=".github/readme-assets/thinking-machines.svg" alt="" width="18" height="18" align="texttop" /> Tinker,
<img src=".github/readme-assets/ray.svg" alt="" width="18" height="18" align="texttop" /> Ray,
<img src=".github/readme-assets/huggingface.svg" alt="" width="18" height="18" align="texttop" /> Hugging Face Jobs를 사용할 수 있습니다.

**에이전트가 모든 실험을 기억하게 하세요.** PG Lab AI는 각 실험을 로컬 SQL 데이터베이스에 기록하고 로그, 코드,
산출물을 사용자 컴퓨터에 보관합니다. 에이전트는 이전 결과에서 배우고
다음에 무엇을 시도할지 결정할 수 있습니다. 데스크톱 앱에서는 에이전트가 한 작업을
쉽게 확인하고 각 결과의 근거를 살펴볼 수 있습니다.

**연구를 직접 보관하고 소유하세요.** 모든 작업을 로컬에서 실행할 수 있습니다. 코드나 에이전트 실행 기록을 수집하지 않습니다.
프로젝트, 대화, 실험, 로그, 산출물은 사용자의 컴퓨터에 저장되며
사용자가 관리합니다.

## 사용 분석

PG Lab AI는 사용 분석을 수집하지 않으며 자체 서버도 없습니다. 데이터는 코딩 에이전트의 모델 제공업체와 검색하는 문헌 서비스(alphaXiv, PubMed, OpenAlex, bioRxiv)처럼 직접 선택해 사용하는 서비스에만 전송됩니다.
