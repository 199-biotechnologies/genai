# genai

AI image and video generation from the terminal. One command, multiple providers.

```bash
genai image "a cat in space"
genai video "ocean waves at sunset"
```

## Install

```bash
cargo install genai-media
```

Or from source:

```bash
git clone https://github.com/199-biotechnologies/genai
cd genai && cargo install --path .
```

## Setup

Set your API key as an environment variable:

```bash
# fal.ai (primary — 1000+ models, image + video)
export FAL_KEY=your-key-here

# OpenAI (fallback — gpt-image-1)
export OPENAI_API_KEY=your-key-here
```

Get a fal.ai key at [fal.ai/dashboard/keys](https://fal.ai/dashboard/keys).

## Usage

### Image Generation

```bash
genai image "a majestic horse at sunset"
genai image "minimalist logo" --model flux-dev --size 1024x1024
genai image "portrait photo" --provider openai --output portrait.png
```

### Video Generation

```bash
genai video "drone shot of a forest"
genai video "waves crashing" --model minimax-video --duration 10
genai video "city timelapse" --aspect-ratio 9:16
```

### List Models

```bash
genai models
genai models --type video
```

## Providers

| Provider | Image Models | Video Models | Key |
|----------|-------------|-------------|-----|
| **fal.ai** | Flux Pro, Flux Dev, Flux Schnell | Kling 2.0, Minimax | `FAL_KEY` |
| **OpenAI** | gpt-image-1 | Sora (pending) | `OPENAI_API_KEY` |

Provider is auto-detected from which API key is set. Override with `--provider fal` or `--provider openai`.

## Agent Integration

Built with [agent-cli-framework](https://github.com/199-biotechnologies/agent-cli-framework) patterns:

```bash
genai agent-info          # Machine-readable capability manifest
genai skill install       # Install skill for Claude Code / Gemini CLI
genai agent-info | jq     # Pipe-friendly JSON output
```

## Output

- **Terminal**: Spinner with progress, saves to `./<slug>-<timestamp>.png`
- **Piped**: JSON with status, file path, provider, model, timing
- **--output**: Override output file path
- **--json**: Force JSON output in terminal

## License

MIT — [199 Biotechnologies](https://github.com/199-biotechnologies)
