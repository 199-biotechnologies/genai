use crate::error::GenaiError;
use crate::output;
use crate::providers::{ImageRequest, MediaOutput, VideoRequest};
use serde::Deserialize;
use std::time::{Duration, Instant};

pub struct FalProvider {
    pub api_key: String,
}

impl FalProvider {
    pub fn from_env() -> Result<Self, GenaiError> {
        let api_key = std::env::var("FAL_KEY")
            .map_err(|_| GenaiError::Config("FAL_KEY environment variable not set".into()))?;
        Ok(Self { api_key })
    }

    fn auth_header(&self) -> String {
        format!("Key {}", self.api_key)
    }

    async fn queue_run(
        &self,
        client: &reqwest::Client,
        model_id: &str,
        body: serde_json::Value,
    ) -> Result<serde_json::Value, GenaiError> {
        let url = format!("https://queue.fal.run/{model_id}");

        output::print_status("Submitting to fal.ai...");
        let resp = client
            .post(&url)
            .header("Authorization", self.auth_header())
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| GenaiError::Transient(format!("fal.ai submit failed: {e}")))?;

        let status = resp.status();
        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            return Err(GenaiError::RateLimited("fal.ai rate limit hit".into()));
        }
        if !status.is_success() && status != reqwest::StatusCode::ACCEPTED {
            let text = resp.text().await.unwrap_or_default();
            return Err(GenaiError::Transient(format!(
                "fal.ai returned HTTP {status}: {text}"
            )));
        }

        let queue_resp: QueueSubmitResponse = resp
            .json()
            .await
            .map_err(|e| GenaiError::Transient(format!("Failed to parse queue response: {e}")))?;

        let status_url = queue_resp.status_url;
        let response_url = queue_resp.response_url;

        loop {
            tokio::time::sleep(Duration::from_secs(2)).await;

            let poll = client
                .get(&status_url)
                .header("Authorization", self.auth_header())
                .send()
                .await
                .map_err(|e| GenaiError::Transient(format!("fal.ai poll failed: {e}")))?;

            let poll_body: QueueStatusResponse = poll
                .json()
                .await
                .map_err(|e| GenaiError::Transient(format!("Failed to parse poll response: {e}")))?;

            match poll_body.status.as_str() {
                "IN_QUEUE" => {
                    let pos = poll_body.queue_position.unwrap_or(0);
                    output::print_status(&format!("Queued (position {pos})..."));
                }
                "IN_PROGRESS" => {
                    output::print_status("Processing...");
                }
                "COMPLETED" => break,
                "FAILED" => {
                    return Err(GenaiError::Transient(
                        "fal.ai request failed during processing".into(),
                    ));
                }
                other => {
                    output::print_status(&format!("Status: {other}..."));
                }
            }
        }

        let result = client
            .get(&response_url)
            .header("Authorization", self.auth_header())
            .send()
            .await
            .map_err(|e| GenaiError::Transient(format!("fal.ai result fetch failed: {e}")))?;

        result
            .json()
            .await
            .map_err(|e| GenaiError::Transient(format!("Failed to parse fal.ai result: {e}")))
    }

    pub async fn generate_image(
        &self,
        client: &reqwest::Client,
        req: &ImageRequest,
        explicit_output: Option<&str>,
    ) -> Result<MediaOutput, GenaiError> {
        let start = Instant::now();

        let image_size = match (req.width, req.height) {
            (1024, 1024) => "square_hd",
            (w, h) if w > h => "landscape_16_9",
            (w, h) if h > w => "portrait_16_9",
            _ => "square_hd",
        };

        let body = serde_json::json!({
            "prompt": req.prompt,
            "image_size": image_size,
            "num_images": 1,
            "output_format": "png",
        });

        let result = self.queue_run(client, &req.model_id, body).await?;

        let url = result["images"][0]["url"]
            .as_str()
            .ok_or_else(|| GenaiError::Transient("No image URL in fal.ai response".into()))?;

        let out_path = output::output_path(explicit_output, &req.prompt, "png");
        output::print_status("Downloading...");
        output::download_file(client, url, &out_path).await?;

        Ok(MediaOutput {
            local_path: out_path,
            provider: "fal".into(),
            model: req.model_id.rsplit('/').next().unwrap_or(&req.model_id).into(),
            elapsed: start.elapsed(),
        })
    }

    pub async fn generate_video(
        &self,
        client: &reqwest::Client,
        req: &VideoRequest,
        explicit_output: Option<&str>,
    ) -> Result<MediaOutput, GenaiError> {
        let start = Instant::now();

        let body = serde_json::json!({
            "prompt": req.prompt,
            "duration": req.duration_secs.to_string(),
            "aspect_ratio": req.aspect_ratio,
        });

        let result = self.queue_run(client, &req.model_id, body).await?;

        let url = result["video"]["url"]
            .as_str()
            .ok_or_else(|| GenaiError::Transient("No video URL in fal.ai response".into()))?;

        let out_path = output::output_path(explicit_output, &req.prompt, "mp4");
        output::print_status("Downloading...");
        output::download_file(client, url, &out_path).await?;

        Ok(MediaOutput {
            local_path: out_path,
            provider: "fal".into(),
            model: req.model_id.rsplit('/').next().unwrap_or(&req.model_id).into(),
            elapsed: start.elapsed(),
        })
    }
}

#[derive(Deserialize)]
struct QueueSubmitResponse {
    #[allow(dead_code)]
    request_id: String,
    response_url: String,
    status_url: String,
}

#[derive(Deserialize)]
struct QueueStatusResponse {
    status: String,
    queue_position: Option<u32>,
}
