use crate::providers::MediaType;

#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub name: &'static str,
    pub provider: &'static str,
    pub media_type: MediaType,
    pub model_id: &'static str,
    pub description: &'static str,
    pub default: bool,
}

pub fn all_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            name: "flux-pro",
            provider: "fal",
            media_type: MediaType::Image,
            model_id: "fal-ai/flux-pro/v1.1",
            description: "FLUX Pro 1.1 — best quality general-purpose",
            default: true,
        },
        ModelInfo {
            name: "flux-dev",
            provider: "fal",
            media_type: MediaType::Image,
            model_id: "fal-ai/flux/dev",
            description: "FLUX Dev — fast, good quality",
            default: false,
        },
        ModelInfo {
            name: "flux-schnell",
            provider: "fal",
            media_type: MediaType::Image,
            model_id: "fal-ai/flux/schnell",
            description: "FLUX Schnell — fastest, lower quality",
            default: false,
        },
        ModelInfo {
            name: "kling-video",
            provider: "fal",
            media_type: MediaType::Video,
            model_id: "fal-ai/kling-video/v2/master/text-to-video",
            description: "Kling 2.0 Master — strong motion, 1080p",
            default: true,
        },
        ModelInfo {
            name: "minimax-video",
            provider: "fal",
            media_type: MediaType::Video,
            model_id: "fal-ai/minimax-video",
            description: "Minimax — fast, good quality video",
            default: false,
        },
        ModelInfo {
            name: "gpt-image-1",
            provider: "openai",
            media_type: MediaType::Image,
            model_id: "gpt-image-1",
            description: "OpenAI GPT Image 1 — high quality",
            default: true,
        },
        ModelInfo {
            name: "sora",
            provider: "openai",
            media_type: MediaType::Video,
            model_id: "sora",
            description: "OpenAI Sora — video generation (API pending)",
            default: true,
        },
    ]
}

pub fn resolve_model(
    name: Option<&str>,
    provider: &str,
    media_type: MediaType,
) -> Option<ModelInfo> {
    let models = all_models();
    if let Some(name) = name {
        models
            .iter()
            .find(|m| m.name == name && m.provider == provider && m.media_type == media_type)
            .cloned()
            .or_else(|| {
                models
                    .iter()
                    .find(|m| m.name == name && m.media_type == media_type)
                    .cloned()
            })
    } else {
        models
            .iter()
            .find(|m| m.provider == provider && m.media_type == media_type && m.default)
            .cloned()
    }
}
