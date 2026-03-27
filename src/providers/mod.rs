pub mod fal;
pub mod openai;

use crate::error::GenaiError;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MediaType {
    Image,
    Video,
}

#[derive(Debug)]
pub struct ImageRequest {
    pub prompt: String,
    pub model_id: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub struct VideoRequest {
    pub prompt: String,
    pub model_id: String,
    pub aspect_ratio: String,
    pub duration_secs: u8,
}

#[derive(Debug)]
pub struct MediaOutput {
    pub local_path: PathBuf,
    pub provider: String,
    pub model: String,
    pub elapsed: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProviderKind {
    Fal,
    OpenAi,
}

pub fn select_provider(explicit: Option<&str>) -> Result<ProviderKind, GenaiError> {
    if let Some(name) = explicit {
        return match name {
            "fal" => Ok(ProviderKind::Fal),
            "openai" => Ok(ProviderKind::OpenAi),
            other => Err(GenaiError::BadInput(format!(
                "Unknown provider '{other}'. Use 'fal' or 'openai'"
            ))),
        };
    }
    if std::env::var("FAL_KEY").ok().filter(|v| !v.is_empty()).is_some() {
        return Ok(ProviderKind::Fal);
    }
    if std::env::var("OPENAI_API_KEY").ok().filter(|v| !v.is_empty()).is_some() {
        return Ok(ProviderKind::OpenAi);
    }
    Err(GenaiError::Config(
        "No API key found. Set FAL_KEY or OPENAI_API_KEY".into(),
    ))
}
