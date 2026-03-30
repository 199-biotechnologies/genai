<div align="center">

# GenAI

**Generate images and videos from your terminal. One command, any model.**

<br />

[![Star this repo](https://img.shields.io/github/stars/199-biotechnologies/genai?style=for-the-badge&logo=github&label=%E2%AD%90%20Star%20this%20repo&color=yellow)](https://github.com/199-biotechnologies/genai/stargazers)
&nbsp;&nbsp;
[![Follow @longevityboris](https://img.shields.io/badge/Follow_%40longevityboris-000000?style=for-the-badge&logo=x&logoColor=white)](https://x.com/longevityboris)

<br />

[![Crates.io](https://img.shields.io/crates/v/genai-media?style=for-the-badge&logo=rust&logoColor=white&label=crates.io)](https://crates.io/crates/genai-media)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)](LICENSE)
[![Homebrew](https://img.shields.io/badge/homebrew-tap-orange?style=for-the-badge&logo=homebrew&logoColor=white)](https://github.com/199-biotechnologies/homebrew-tap)

---

A fast, single-binary CLI for AI image and video generation. Supports 10+ models across fal.ai and OpenAI. Pick a model, write a prompt, get your file. Works standalone or as a skill inside Claude Code, Gemini CLI, and other AI agents.

[Install](#install) | [How It Works](#how-it-works) | [Models](#supported-models) | [Contributing](#contributing)

</div>

## Why This Exists

You want to generate an image or video from the command line. You do not want to open a browser, manage SDKs, or write boilerplate API calls. You want one command that works with Flux, Kling, Veo, GPT Image, and more.

GenAI gives you that. Set an API key, run `genai image "your prompt"`, and get a file on disk. Switch models with a flag. Pipe JSON output into scripts. Let your AI coding agent call it as a skill.

## Install

### Homebrew (macOS / Linux)

```bash
brew tap 199-biotechnologies/tap
brew install genai
```

### Cargo (all platforms)

```bash
cargo install genai-media
```

### From source

```bash
git clone https://github.com/199-biotechnologies/genai
cd genai && cargo install --path .
```

### Set your API key

```bash
# fal.ai — 10+ image and video models
export FAL_KEY=your-key-here

# OpenAI — gpt-image-1
export OPENAI_API_KEY=your-key-here
```

Get a fal.ai key at [fal.ai/dashboard/keys](https://fal.ai/dashboard/keys). Get an OpenAI key at [platform.openai.com/api-keys](https://platform.openai.com/api-keys).

## How It Works

```
prompt ──> genai ──> provider API ──> file on disk
                  │
                  ├── auto-detects provider from your API key
                  ├── resolves model (or uses your --model flag)
                  └── saves .png or .mp4 with spinner + progress
```

1. You pass a text prompt and optional flags (model, size, duration, output path)
2. GenAI picks the provider based on which API key you have set (or your `--provider` flag)
3. It calls the API, downloads the result, and saves it to disk
4. In a pipe or with `--json`, it outputs structured JSON for scripting

## Supported Models

### Image Models

| Model | Provider | Description |
|-------|----------|-------------|
| **nano-banana-2** (default) | fal.ai | Nano Banana 2 (Google) -- best quality, text rendering |
| flux-2-pro | fal.ai | FLUX 2 Pro -- fast, high quality |
| flux-pro | fal.ai | FLUX Pro 1.1 -- general-purpose |
| flux-dev | fal.ai | FLUX Dev -- fast, good quality |
| flux-schnell | fal.ai | FLUX Schnell -- fastest, lower quality |
| **gpt-image-1** (default) | OpenAI | GPT Image 1 -- high quality |

### Video Models

| Model | Provider | Description |
|-------|----------|-------------|
| **kling-video** (default) | fal.ai | Kling 3.0 Pro -- cinematic motion, 1080p |
| veo3 | fal.ai | Veo 3.1 (Google) -- highest quality, 4K |
| ltx-video | fal.ai | LTX Video 2.0 Pro -- fast, affordable ($0.02/video) |
| minimax-video | fal.ai | Minimax -- fast, good quality |
| **sora** (default) | OpenAI | Sora -- video generation (API pending) |

Run `genai models` to see all available models. Filter with `genai models --type image` or `genai models --type video`.

## Usage Examples

### Generate images

```bash
# Use the default model
genai image "a majestic horse at sunset"

# Pick a specific model and size
genai image "minimalist logo" --model flux-dev --size 1024x1024

# Force a provider and save to a specific file
genai image "portrait photo" --provider openai --output portrait.png
```

### Generate videos

```bash
# Default model, 5-second video
genai video "drone shot of a forest"

# Specific model, longer duration
genai video "waves crashing" --model minimax-video --duration 10

# Vertical video for social media
genai video "city timelapse" --aspect-ratio 9:16
```

### Scripting and automation

```bash
# JSON output for pipelines
genai image "test" --json | jq '.file_path'

# List models as JSON
genai models --json
```

### AI agent integration

GenAI works as a skill inside Claude Code, Gemini CLI, and other agent frameworks.

```bash
# Install the skill for your AI agent
genai skill install

# Machine-readable capability manifest
genai agent-info | jq
```

## Output Modes

| Context | Behavior |
|---------|----------|
| **Terminal** | Spinner with progress. Saves to `./<slug>-<timestamp>.png` or `.mp4` |
| **Piped / --json** | Structured JSON with status, file path, provider, model, timing |
| **--output** | Override the output file path |

## Contributing

Contributions are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) for setup instructions and guidelines.

The fastest way to contribute: add a new model to `src/models.rs` and open a PR.

## License

MIT -- see [LICENSE](LICENSE) for details.

---
<div align="center">

Built by [Boris Djordjevic](https://github.com/longevityboris) at [199 Biotechnologies](https://github.com/199-biotechnologies) | [Paperfoot AI](https://paperfoot.ai)

<br />

**If this is useful to you:**

[![Star this repo](https://img.shields.io/github/stars/199-biotechnologies/genai?style=for-the-badge&logo=github&label=%E2%AD%90%20Star%20this%20repo&color=yellow)](https://github.com/199-biotechnologies/genai/stargazers)
&nbsp;&nbsp;
[![Follow @longevityboris](https://img.shields.io/badge/Follow_%40longevityboris-000000?style=for-the-badge&logo=x&logoColor=white)](https://x.com/longevityboris)

</div>
