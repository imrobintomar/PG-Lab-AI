<div align="center">

<h1><img src=".github/readme-assets/pg-lab-ai.png" alt="" width="56" align="absmiddle" /> PG Lab AI</h1>

Un espacio de trabajo impulsado por IA para el descubrimiento científico.

<p>
<a href="https://github.com/imrobintomar/PG-Lab-AI/releases/latest"><img src="https://img.shields.io/github/v/release/imrobintomar/PG-Lab-AI?style=flat-square" alt="Latest release" /></a>
<a href="https://github.com/imrobintomar/PG-Lab-AI/blob/main/LICENSE"><img src="https://img.shields.io/github/license/imrobintomar/PG-Lab-AI?style=flat-square" alt="License" /></a>
</p>

<p><a href="README.md">English</a> · <a href="README.es.md">Español</a> · <a href="README.ko.md">한국어</a> · <a href="README.zh.md">简体中文</a> · <a href="README.ja.md">日本語</a></p>

<p><img src=".github/readme-assets/pg-lab-ai-screenshot.png" alt="PG Lab AI desktop app running a research agent on a local Qwen model through Ollama" width="680" /></p>

<hr />

<p>Tanto si buscas un copiloto como una herramienta de investigación autónoma, PG Lab AI acelerará tu trabajo. Pon en marcha agentes de investigación capaces de revisar bibliografía, formular hipótesis, ejecutar experimentos y generar resultados de investigación.</p>

<p>
<a href="https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/PG-Lab-AI.dmg"><picture><source media="(prefers-color-scheme: dark)" srcset=".github/readme-assets/download-macos-dark.svg"><img src=".github/readme-assets/download-macos.svg" alt="Download PG Lab AI for macOS" width="220" height="44" /></picture></a>
<a href="https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/PG-Lab-AI-Setup.exe"><picture><source media="(prefers-color-scheme: dark)" srcset=".github/readme-assets/download-windows-dark.svg"><img src=".github/readme-assets/download-windows.svg" alt="Download PG Lab AI for Windows (Beta)" width="220" height="44" /></picture></a>
<a href="https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/PG-Lab-AI-x86_64.AppImage"><picture><source media="(prefers-color-scheme: dark)" srcset=".github/readme-assets/download-linux-dark.svg"><img src=".github/readme-assets/download-linux.svg" alt="Download PG Lab AI for Linux" width="220" height="44" /></picture></a>
</p>

<p>
<a href="https://github.com/imrobintomar/PG-Lab-AI/tree/main/docs"><img src=".github/readme-assets/action-documentation.svg" alt="Documentation" width="132" height="24" /></a><img src=".github/readme-assets/action-separator.svg" alt=" · " width="12" height="24" />
<a href="https://github.com/imrobintomar/PG-Lab-AI/releases"><img src=".github/readme-assets/action-releases.svg" alt="Releases" width="78" height="24" /></a>
</p>

<p><sub>macOS 11+ · La versión beta para Windows requiere <a href="docs/windows.md">Git for Windows</a> · La app para <a href="docs/linux.md">Linux</a> requiere glibc 2.35+</sub></p>

</div>

## Primeros pasos

**Recomendamos usar la aplicación de escritorio de PG Lab AI.** Descárgala para tu plataforma:

| Plataforma | Descarga |
| --- | --- |
| macOS 11+ (Apple silicon e Intel) | [PG-Lab-AI.dmg](https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/PG-Lab-AI.dmg) |
| Windows 10/11 (Beta) | [PG-Lab-AI-Setup.exe](https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/PG-Lab-AI-Setup.exe) |
| Linux x86_64 | [PG-Lab-AI-x86_64.AppImage](https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/PG-Lab-AI-x86_64.AppImage) |
| Linux ARM64 | [PG-Lab-AI-aarch64.AppImage](https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/PG-Lab-AI-aarch64.AppImage) |
| Línea de comandos (macOS/Linux) | `curl -LsSf https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/pg-lab-ai-cli-installer.sh \| sh` |
| Línea de comandos (Windows PowerShell) | `powershell -ExecutionPolicy Bypass -c "irm https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/pg-lab-ai-cli-installer.ps1 \| iex"` |

Versión actual: **[v1.0.0](https://github.com/imrobintomar/PG-Lab-AI/releases/tag/v1.0.0)** · [Todas las versiones](https://github.com/imrobintomar/PG-Lab-AI/releases) · Los enlaces de arriba siempre descargan la versión más reciente.

<details>
<summary>¿Prefieres usar solo la CLI? Instálala en macOS o Linux</summary>

```sh
curl -LsSf https://github.com/imrobintomar/PG-Lab-AI/releases/latest/download/pg-lab-ai-cli-installer.sh | sh
orx up
```

`orx up` abre el panel local en `http://127.0.0.1:4791`.

También puedes instalar `orx` para tu terminal desde
Settings → Updates → **Install the `orx` command** en la aplicación.

</details>

<details>
<summary>Primer inicio: las aplicaciones aún no están firmadas</summary>

- **macOS:** haz clic derecho en PG Lab AI en Aplicaciones, elige **Abrir** y luego **Abrir** otra vez.
- **Windows:** en el aviso de SmartScreen, elige **Más información** → **Ejecutar de todas formas**.
- **Linux:** haz ejecutable el AppImage (`chmod +x PG-Lab-AI-*.AppImage`) y ejecútalo.

En un Mac administrado, las políticas de gestión de dispositivos pueden bloquear por completo las aplicaciones sin firmar.

</details>

## Cómo funciona

**Usa tu agente de programación favorito.** PG Lab AI se integra de forma nativa con <img src=".github/readme-assets/claude.svg" alt="" width="18" height="18" align="texttop" /> Claude Code,
<img src=".github/readme-assets/codex.svg" alt="" width="18" height="18" align="texttop" /> Codex,
<img src=".github/readme-assets/opencode.svg" alt="" width="18" height="18" align="texttop" /> OpenCode,
<img src=".github/readme-assets/cursor.svg" alt="" width="18" height="18" align="texttop" /> Cursor y
<img src=".github/readme-assets/antigravity.svg" alt="" width="18" height="18" align="texttop" /> Google Antigravity.
También puedes [usar un modelo local](docs/local-models.md) con OpenCode a través de
<img src=".github/readme-assets/lmstudio.svg" alt="" width="18" height="18" align="texttop" /> LM Studio,
<img src=".github/readme-assets/omlx.svg" alt="" width="18" height="18" align="texttop" /> oMLX,
<img src=".github/readme-assets/ollama.svg" alt="" width="18" height="18" align="texttop" /> Ollama o un endpoint personalizado.
**Local por defecto:** si Ollama está en ejecución con `qwen3.8-cc:27b-q4_K_M`, PG Lab AI
lo conecta automáticamente y lo usa en los chats nuevos, sin necesidad de una cuenta en la nube.

**Fundamenta tus hipótesis en la bibliografía más reciente.** PG Lab AI se integra con <img src=".github/readme-assets/alphaxiv.svg" alt="" width="18" height="18" align="absmiddle" /> alphaXiv, <img src=".github/readme-assets/biorxiv.svg" alt="" width="18" height="18" align="absmiddle" /> bioRxiv, <img src=".github/readme-assets/pubmed.svg" alt="" width="18" height="18" align="absmiddle" /> PubMed, Europe PMC y ClinicalTrials.gov para que los agentes encuentren artículos relevantes, formulen hipótesis informadas y conecten sus experimentos con investigaciones existentes.

**Elige dónde ejecutar tus experimentos.** Ejecútalos en local o utiliza SSH,
Slurm,
<img src=".github/readme-assets/kubernetes.svg" alt="" width="18" height="18" align="texttop" /> Kubernetes,
<img src=".github/readme-assets/modal.svg" alt="" width="18" height="18" align="texttop" /> Modal,
<img src=".github/readme-assets/thinking-machines.svg" alt="" width="18" height="18" align="texttop" /> Tinker,
<img src=".github/readme-assets/ray.svg" alt="" width="18" height="18" align="texttop" /> Ray y
<img src=".github/readme-assets/huggingface.svg" alt="" width="18" height="18" align="texttop" /> Hugging Face Jobs.

**Dale a tus agentes una memoria de cada experimento.** PG Lab AI registra cada experimento en una base de datos SQL local y conserva sus logs, código
y artefactos en tu equipo para que los agentes puedan
aprender de los resultados anteriores y decidir qué probar después. La aplicación de escritorio
te permite ver fácilmente lo que han hecho tus agentes y examinar la evidencia de cada resultado.

**Tu investigación te pertenece.** Ejecuta todo en local. No recopilamos tu código ni las trazas de tus agentes.
Tus proyectos, conversaciones, experimentos, registros y artefactos permanecen en tu equipo,
bajo tu control.

## Analíticas de uso

PG Lab AI no recopila analíticas de uso y no tiene servidor propio. Solo reciben
datos los servicios que eliges usar: el proveedor de modelos de tu agente de
programación y las fuentes bibliográficas que consultas (alphaXiv, PubMed, OpenAlex, bioRxiv).
