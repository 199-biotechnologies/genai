# genai CLI — Design Spec

**Date**: 2026-03-27
**Status**: Approved
**Repo**: `199-biotechnologies/genai`

## Purpose

A Rust CLI for AI image and video generation across multiple providers, following agent-cli-framework patterns. Primary provider: fal.ai (1000+ models). Fallback: OpenAI. No existing unified CLI like this exists in any language.

## Commands

```
genai image "prompt" [options]    # Generate an image
genai video "prompt" [options]    # Generate a video
genai models [--type image|video] # List available models
genai agent-info                  # Machine-readable capability manifest (JSON)
genai skill install|status        # Install/check skill files for AI agent platforms
genai update [--check]            # Self-update from GitHub Releases
```

### Image Options

| Flag | Default | Description |
|------|---------|-------------|
| `--model` | `flux-pro` (fal) / `gpt-image-1` (openai) | Model to use |
| `--provider` | auto-detect from env | `fal` or `openai` |
| `--output` | `./<slug>-<timestamp>.png` | Output file path |
| `--size` | `1024x1024` | Image dimensions |
| `--json` | false (auto when piped) | Force JSON output |

### Video Options

| Flag | Default | Description |
|------|---------|-------------|
| `--model` | `kling-video` (fal) / `sora` (openai) | Model to use |
| `--provider` | auto-detect from env | `fal` or `openai` |
| `--output` | `./<slug>-<timestamp>.mp4` | Output file path |
| `--duration` | `5` | Duration in seconds |
| `--size` | `1280x720` | Video dimensions |
| `--json` | false (auto when piped) | Force JSON output |

## Provider Selection Logic

Evaluated in order:

1. `--provider fal` or `--provider openai` → use specified provider
2. `FAL_KEY` env var present → use fal.ai
3. `OPENAI_API_KEY` env var present → use OpenAI
4. Neither → exit code 2 with suggestion: `"Set FAL_KEY or OPENAI_API_KEY. Get a fal.ai key at https://fal.ai/dashboard/keys"`

No config file. Env vars only. Keep it simple.

## Default Models

### fal.ai

| Task | Model ID | Why |
|------|----------|-----|
| Image | `fal-ai/flux-pro/v1.1` | Best quality general-purpose, fast |
| Video | `fal-ai/kling-video/v2/master` | Strong motion, 1080p, 5-10s clips |

### OpenAI

| Task | Model ID | Why |
|------|----------|-----|
| Image | `gpt-image-1` | Latest OpenAI image model |
| Video | `sora` | OpenAI's video model (API availability TBD) |

Users override with `--model <name>`. The `genai models` command lists all available options with short descriptions.

## API Integration

### fal.ai Flow

fal.ai uses an async queue pattern:

1. **Submit**: `POST https://queue.fal.run/{model_id}` with JSON body `{"prompt": "...", "image_size": "..."}` + `Authorization: Key {FAL_KEY}`
2. **Poll**: `GET https://queue.fal.run/{model_id}/requests/{request_id}/status` until `status == "COMPLETED"`
3. **Result**: Response contains `{"images": [{"url": "https://..."}]}` (image) or `{"video": {"url": "https://..."}}` (video)
4. **Download**: GET the URL, write to disk

### OpenAI Flow

OpenAI is synchronous for images:

1. **Image**: `POST https://api.openai.com/v1/images/generations` with `{"model": "gpt-image-1", "prompt": "...", "size": "..."}` + `Authorization: Bearer {OPENAI_API_KEY}`
2. **Response**: `{"data": [{"url": "https://..."}]}` or base64 in `b64_json`
3. **Download**: GET the URL or decode base64, write to disk

Video (Sora) endpoint TBD — implement when API stabilizes.

## Output Behavior

### Terminal (TTY)

```
⠋ Generating image with flux-pro via fal.ai...
⠙ Queued (position 3)...
⠹ Processing...
✓ Saved to ./cat-in-space-1711547200.png (4.2s, fal.ai/flux-pro)
```

### Piped / --json

```json
{
  "status": "ok",
  "file": "./cat-in-space-1711547200.png",
  "provider": "fal",
  "model": "flux-pro",
  "elapsed_ms": 4200,
  "prompt": "a cat in space"
}
```

### Error (JSON)

```json
{
  "status": "error",
  "code": 4,
  "error": "Rate limited by fal.ai",
  "suggestion": "Wait 60 seconds or switch provider with --provider openai"
}
```

## Semantic Exit Codes

| Code | Meaning | Example |
|------|---------|---------|
| 0 | Success | Image generated and saved |
| 1 | Transient error | Network timeout, provider 500 |
| 2 | Configuration error | No API key set |
| 3 | Bad input | Empty prompt, invalid size format |
| 4 | Rate limited | Provider rate limit hit |

## File Naming

Auto-generated output filenames: `./<slug>-<unix-timestamp>.<ext>`

- Slug: first 40 chars of prompt, lowercased, non-alphanumeric replaced with `-`, consecutive dashes collapsed
- Extension: `.png` for images, `.mp4` for videos
- Example: `a-cat-in-space-1711547200.png`

## Architecture

```
src/
  main.rs          # CLI parsing (clap), format detection, dispatch
  providers/
    mod.rs         # MediaProvider trait + provider selection logic
    fal.rs         # fal.ai implementation (submit → poll → download)
    openai.rs      # OpenAI implementation (sync request → download)
  models.rs        # Model registry (name, fal_id, type, description)
  output.rs        # File naming, download, TTY/JSON output formatting
  agent.rs         # agent-info manifest + skill install
```

### Core Trait

```rust
#[async_trait]
pub trait MediaProvider {
    async fn generate_image(&self, req: &ImageRequest) -> Result<MediaOutput>;
    async fn generate_video(&self, req: &VideoRequest) -> Result<MediaOutput>;
    fn name(&self) -> &str;
    fn list_models(&self, media_type: Option<MediaType>) -> Vec<ModelInfo>;
}
```

### Key Types

```rust
pub struct ImageRequest {
    pub prompt: String,
    pub model: String,
    pub size: (u32, u32),
}

pub struct VideoRequest {
    pub prompt: String,
    pub model: String,
    pub size: (u32, u32),
    pub duration_secs: u8,
}

pub struct MediaOutput {
    pub url: String,          // Remote URL before download
    pub local_path: PathBuf,  // Where file was saved
    pub provider: String,
    pub model: String,
    pub elapsed: Duration,
}
```

## Dependencies

```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
reqwest = { version = "0.12", features = ["json"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
owo-colors = "4"
thiserror = "2"
self_update = { version = "0.42", features = ["archive-tar", "compression-flate2"] }
```

No async-openai, no fal-rs. Direct HTTP only. Minimal dependency tree.

## agent-info Manifest

```json
{
  "name": "genai",
  "version": "0.1.0",
  "description": "AI image and video generation CLI",
  "commands": {
    "image": {
      "description": "Generate an image from a text prompt",
      "usage": "genai image \"prompt\" [--model MODEL] [--size WxH] [--output PATH]",
      "required_args": ["prompt"]
    },
    "video": {
      "description": "Generate a video from a text prompt",
      "usage": "genai video \"prompt\" [--model MODEL] [--duration SECS] [--output PATH]",
      "required_args": ["prompt"]
    },
    "models": {
      "description": "List available AI models",
      "usage": "genai models [--type image|video]"
    }
  },
  "exit_codes": {
    "0": "success",
    "1": "transient error (retry may help)",
    "2": "configuration error (missing API key)",
    "3": "bad input (invalid prompt or options)",
    "4": "rate limited (wait and retry)"
  },
  "env_vars": {
    "FAL_KEY": "fal.ai API key (primary provider)",
    "OPENAI_API_KEY": "OpenAI API key (fallback provider)"
  }
}
```

## Estimated Size

~600-800 lines of Rust total. Single binary, no runtime dependencies.

## Out of Scope (v0.1)

- Audio/music generation
- Image-to-image / image editing
- Config file (env vars are sufficient)
- Streaming/progress percentage (just spinner)
- Batch generation (one at a time)
- Provider-specific advanced params (LoRA, controlnet, etc.)
