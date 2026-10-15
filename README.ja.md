<div align="center">

<h1><img src=".github/readme-assets/pg-lab-ai.png" alt="" width="56" align="absmiddle" /> PG Lab AI</h1>

科学的発見のための AI 搭載ワークスペース。

<p>
<a href="https://github.com/imrobintomar/PG-Lab-AI/releases/latest"><img src="https://img.shields.io/github/v/release/imrobintomar/PG-Lab-AI?style=flat-square" alt="Latest release" /></a>
<a href="https://github.com/imrobintomar/PG-Lab-AI/blob/main/LICENSE"><img src="https://img.shields.io/github/license/imrobintomar/PG-Lab-AI?style=flat-square" alt="License" /></a>
</p>

<p><a href="README.md">English</a> · <a href="README.es.md">Español</a> · <a href="README.ko.md">한국어</a> · <a href="README.zh.md">简体中文</a> · <a href="README.ja.md">日本語</a></p>

<p><img src=".github/readme-assets/pg-lab-ai-screenshot.png" alt="PG Lab AI desktop app running a research agent on a local Qwen model through Ollama" width="680" /></p>

<hr />

<p>コパイロットを探している方にも、自律研究ツールを探している方にも、PG Lab AI は作業を加速します。 文献のレビュー、仮説の構築、実験の実行、研究成果の作成を行う研究エージェントを起動しましょう。</p>

<p>
<a href="https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/PG-Lab-AI.dmg"><picture><source media="(prefers-color-scheme: dark)" srcset=".github/readme-assets/download-macos-dark.svg"><img src=".github/readme-assets/download-macos.svg" alt="Download PG Lab AI for macOS" width="220" height="44" /></picture></a>
<a href="https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/PG-Lab-AI-Setup.exe"><picture><source media="(prefers-color-scheme: dark)" srcset=".github/readme-assets/download-windows-dark.svg"><img src=".github/readme-assets/download-windows.svg" alt="Download PG Lab AI for Windows（ベータ版）" width="220" height="44" /></picture></a>
<a href="https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/PG-Lab-AI-x86_64.AppImage"><picture><source media="(prefers-color-scheme: dark)" srcset=".github/readme-assets/download-linux-dark.svg"><img src=".github/readme-assets/download-linux.svg" alt="Download PG Lab AI for Linux" width="220" height="44" /></picture></a>
</p>

<p>
<a href="https://github.com/imrobintomar/PG-Lab-AI/tree/main/docs"><img src=".github/readme-assets/action-documentation.svg" alt="Documentation" width="132" height="24" /></a><img src=".github/readme-assets/action-separator.svg" alt=" · " width="12" height="24" />
<a href="https://github.com/imrobintomar/PG-Lab-AI/releases"><img src=".github/readme-assets/action-releases.svg" alt="Releases" width="78" height="24" /></a>
</p>

<p><sub>macOS 11+ · Windows ベータ版には次が必要です： <a href="docs/windows.md">Git for Windows</a> · <a href="docs/linux.md">Linux</a> アプリには glibc 2.35 以上が必要です</sub></p>

</div>

## はじめに

**PG Lab AI デスクトップアプリの利用をおすすめします。** お使いのプラットフォーム向けにダウンロードしてください：

| プラットフォーム | ダウンロード |
| --- | --- |
| macOS 11+（Apple シリコン・Intel） | [PG-Lab-AI.dmg](https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/PG-Lab-AI.dmg) |
| Windows 10/11（ベータ版） | [PG-Lab-AI-Setup.exe](https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/PG-Lab-AI-Setup.exe) |
| Linux x86_64 | [PG-Lab-AI-x86_64.AppImage](https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/PG-Lab-AI-x86_64.AppImage) |
| Linux ARM64 | [PG-Lab-AI-aarch64.AppImage](https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/PG-Lab-AI-aarch64.AppImage) |
| コマンドライン（macOS/Linux） | `curl -LsSf https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/pg-lab-ai-cli-installer.sh \| sh` |
| コマンドライン（Windows PowerShell） | `powershell -ExecutionPolicy Bypass -c "irm https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/pg-lab-ai-cli-installer.ps1 \| iex"` |

現在のリリース：**[v1.1.0](https://github.com/imrobintomar/PG-Lab-AI/releases/tag/v1.1.0)** · [すべてのリリース](https://github.com/imrobintomar/PG-Lab-AI/releases) · 上のリンクは常に最新版をダウンロードします。

<details>
<summary>スタンドアロンの CLI を使いたい場合：macOS または Linux にインストール</summary>

```sh
curl -LsSf https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/pg-lab-ai-cli-installer.sh | sh
orx up
```

`orx up` は `http://127.0.0.1:4791` のローカルダッシュボードを開きます。

アプリの Settings → Updates → **Install the `orx` command** から、ターミナル用の `orx` もインストールできます。

</details>

<details>
<summary>初回起動：アプリはまだコード署名されていません</summary>

- **macOS：** アプリケーションフォルダの PG Lab AI を右クリックして **開く** を選び、もう一度 **開く** を選びます。
- **Windows：** SmartScreen の警告で **詳細情報** → **実行** を選びます。
- **Linux：** AppImage に実行権限を付けて（`chmod +x PG-Lab-AI-*.AppImage`）実行します。

管理対象の Mac では、デバイス管理ポリシーによって未署名のアプリがブロックされる場合があります。

</details>

## 仕組み

**お気に入りのコーディングエージェントを使う。** PG Lab AI は次のツールにネイティブ対応しています： <img src=".github/readme-assets/claude.svg" alt="" width="18" height="18" align="texttop" /> Claude Code,
<img src=".github/readme-assets/codex.svg" alt="" width="18" height="18" align="texttop" /> Codex,
<img src=".github/readme-assets/opencode.svg" alt="" width="18" height="18" align="texttop" /> OpenCode,
<img src=".github/readme-assets/cursor.svg" alt="" width="18" height="18" align="texttop" /> Cursor、
<img src=".github/readme-assets/antigravity.svg" alt="" width="18" height="18" align="texttop" /> Google Antigravity.
OpenCode では、次のツールを通じて[ローカルモデルを使う](docs/local-models.md)こともできます：
<img src=".github/readme-assets/lmstudio.svg" alt="" width="18" height="18" align="texttop" /> LM Studio,
<img src=".github/readme-assets/omlx.svg" alt="" width="18" height="18" align="texttop" /> oMLX,
<img src=".github/readme-assets/ollama.svg" alt="" width="18" height="18" align="texttop" /> Ollama、またはカスタムエンドポイント。
**デフォルトはローカル：** Ollama が `qwen3.8-cc:27b-q4_K_M` を実行していれば、PG Lab AI は自動的に接続し、
新しいチャットで使用します。クラウドのアカウントは不要です。

**最新の文献に基づいて研究仮説を立てる。** PG Lab AI は <img src=".github/readme-assets/alphaxiv.svg" alt="" width="18" height="18" align="absmiddle" /> alphaXiv、<img src=".github/readme-assets/biorxiv.svg" alt="" width="18" height="18" align="absmiddle" /> bioRxiv、<img src=".github/readme-assets/pubmed.svg" alt="" width="18" height="18" align="absmiddle" /> PubMed、Europe PMC、ClinicalTrials.gov と連携し、エージェントが関連論文を見つけ、根拠のある仮説を立て、実験を既存の研究と結び付けられるようにします。

**実行環境を選ぶ。** ローカルで実験を実行するほか、SSH、
Slurm,
<img src=".github/readme-assets/kubernetes.svg" alt="" width="18" height="18" align="texttop" /> Kubernetes,
<img src=".github/readme-assets/modal.svg" alt="" width="18" height="18" align="texttop" /> Modal,
<img src=".github/readme-assets/thinking-machines.svg" alt="" width="18" height="18" align="texttop" /> Tinker,
<img src=".github/readme-assets/ray.svg" alt="" width="18" height="18" align="texttop" /> Ray,
<img src=".github/readme-assets/huggingface.svg" alt="" width="18" height="18" align="texttop" /> Hugging Face Jobs を利用できます。

**すべての実験をエージェントの記憶にする。** PG Lab AI は各実験をローカルの SQL データベースに記録し、ログ、コード、
成果物をあなたのマシンに保存します。エージェントは過去の結果から学び、
次に何を試すかを判断できます。デスクトップアプリでは、エージェントが行った作業と
各結果の根拠を簡単に確認できます。

**研究を手元に保管し、自分で所有する。** すべてローカルで実行できます。コードやエージェントの実行履歴は収集しません。
プロジェクト、会話、実験、ログ、成果物はあなたのマシンに保存され、
あなたが管理します。

## 利用状況の分析

PG Lab AI は利用状況の分析を収集せず、独自のサーバーも持ちません。データを受け取るのは、コーディングエージェントのモデル提供元や、検索する文献サービス（alphaXiv、PubMed、OpenAlex、bioRxiv）など、あなたが選んで使うサービスだけです。
