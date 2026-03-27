use std::path::PathBuf;

const SKILL_CONTENT: &str = r#"---
name: genai
description: Generate AI images and videos from text prompts. Supports fal.ai (Flux, Kling, Minimax) and OpenAI (gpt-image-1). Use when user asks to generate, create, or make images or videos.
---

# genai — AI Image & Video Generation CLI

## Quick Reference

```bash
genai image "prompt"                           # Image with default model (Flux Pro)
genai image "prompt" --model flux-dev          # Specific model
genai video "prompt"                           # Video with default model (Kling)
genai video "prompt" --duration 10             # 10-second video
genai models                                   # List all models
genai models --type video                      # List video models only
```

## Providers

Set `FAL_KEY` for fal.ai (primary) or `OPENAI_API_KEY` for OpenAI (fallback).
Override with `--provider fal` or `--provider openai`.

## Output

Files save to current directory as `<slug>-<timestamp>.png/.mp4`.
Override with `--output path.png`. Outputs JSON when piped.
"#;

pub fn agent_info_json() -> String {
    let version = env!("CARGO_PKG_VERSION");
    serde_json::json!({
        "name": "genai",
        "version": version,
        "description": "AI image and video generation CLI — fal.ai + OpenAI",
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
    })
    .to_string()
}

pub fn skill_install() {
    let dirs = skill_dirs();
    let mut installed = 0;

    for dir in &dirs {
        let skill_dir = dir.join("genai");
        if std::fs::create_dir_all(&skill_dir).is_ok() {
            let path = skill_dir.join("genai.md");
            if std::fs::write(&path, SKILL_CONTENT).is_ok() {
                eprintln!("  Installed: {}", path.display());
                installed += 1;
            }
        }
    }

    if installed == 0 {
        eprintln!("  No agent platforms detected. Checked:");
        for dir in &dirs {
            eprintln!("    {}", dir.display());
        }
    } else {
        eprintln!("  Installed to {installed} platform(s)");
    }
}

pub fn skill_status() {
    let dirs = skill_dirs();
    for dir in &dirs {
        let path = dir.join("genai").join("genai.md");
        let status = if path.exists() { "installed" } else { "not found" };
        eprintln!("  {} — {status}", path.display());
    }
}

fn skill_dirs() -> Vec<PathBuf> {
    let home = dirs::home_dir().unwrap_or_default();
    vec![
        home.join(".claude").join("skills"),
        home.join(".gemini").join("skills"),
    ]
}
