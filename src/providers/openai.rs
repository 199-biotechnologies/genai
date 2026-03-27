use crate::error::GenaiError;
use crate::output;
use crate::providers::{ImageRequest, MediaOutput, VideoRequest};
use serde::Deserialize;
use std::time::Instant;

pub struct OpenAiProvider {
    pub api_key: String,
}

impl OpenAiProvider {
    pub fn from_env() -> Result<Self, GenaiError> {
        let api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| GenaiError::Config("OPENAI_API_KEY environment variable not set".into()))?;
        Ok(Self { api_key })
    }

    fn auth_header(&self) -> String {
        format!("Bearer {}", self.api_key)
    }

    pub async fn generate_image(
        &self,
        client: &reqwest::Client,
        req: &ImageRequest,
        explicit_output: Option<&str>,
    ) -> Result<MediaOutput, GenaiError> {
        let start = Instant::now();

        let size = format!("{}x{}", req.width, req.height);
        let size = match size.as_str() {
            "1024x1024" | "1536x1024" | "1024x1536" => size,
            _ => "1024x1024".to_string(),
        };

        output::print_status("Generating with OpenAI...");

        let body = serde_json::json!({
            "model": req.model_id,
            "prompt": req.prompt,
            "n": 1,
            "size": size,
            "response_format": "b64_json",
        });

        let resp = client
            .post("https://api.openai.com/v1/images/generations")
            .header("Authorization", self.auth_header())
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| GenaiError::Transient(format!("OpenAI request failed: {e}")))?;

        let status = resp.status();
        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            return Err(GenaiError::RateLimited("OpenAI rate limit hit".into()));
        }
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(GenaiError::Transient(format!(
                "OpenAI returned HTTP {status}: {text}"
            )));
        }

        let result: ImageResponse = resp
            .json()
            .await
            .map_err(|e| GenaiError::Transient(format!("Failed to parse OpenAI response: {e}")))?;

        let image_data = result
            .data
            .first()
            .ok_or_else(|| GenaiError::Transient("No image in OpenAI response".into()))?;

        let out_path = output::output_path(explicit_output, &req.prompt, "png");

        use base64::Engine;
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(&image_data.b64_json)
            .map_err(|e| GenaiError::Transient(format!("Failed to decode base64 image: {e}")))?;

        std::fs::write(&out_path, &bytes)
            .map_err(|e| GenaiError::Transient(format!("Failed to write file: {e}")))?;

        Ok(MediaOutput {
            local_path: out_path,
            provider: "openai".into(),
            model: req.model_id.clone(),
            elapsed: start.elapsed(),
        })
    }

    pub async fn generate_video(
        &self,
        _client: &reqwest::Client,
        _req: &VideoRequest,
        _explicit_output: Option<&str>,
    ) -> Result<MediaOutput, GenaiError> {
        Err(GenaiError::BadInput(
            "OpenAI video generation (Sora) is not yet available via API. Use --provider fal instead".into(),
        ))
    }
}

#[derive(Deserialize)]
struct ImageResponse {
    data: Vec<ImageData>,
}

#[derive(Deserialize)]
struct ImageData {
    b64_json: String,
}
