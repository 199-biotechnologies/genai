use crate::error::GenaiError;
use crate::providers::MediaOutput;
use owo_colors::OwoColorize;
use owo_colors::Stream::Stdout;
use serde::Serialize;
use std::io::IsTerminal;
use std::path::{Path, PathBuf};

#[derive(Clone, Copy)]
pub enum Format {
    Json,
    Tty,
}

impl Format {
    pub fn detect(json_flag: bool) -> Self {
        if json_flag || !std::io::stdout().is_terminal() {
            Format::Json
        } else {
            Format::Tty
        }
    }
}

pub fn slugify(prompt: &str) -> String {
    let mut slug = String::with_capacity(40);
    let mut last_was_dash = true;
    for ch in prompt.chars().take(60) {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            last_was_dash = false;
        } else if !last_was_dash {
            slug.push('-');
            last_was_dash = true;
        }
        if slug.len() >= 40 {
            break;
        }
    }
    while slug.ends_with('-') {
        slug.pop();
    }
    if slug.is_empty() {
        slug.push_str("output");
    }
    slug
}

pub fn output_path(explicit: Option<&str>, prompt: &str, ext: &str) -> PathBuf {
    if let Some(p) = explicit {
        return PathBuf::from(p);
    }
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    PathBuf::from(format!("{}-{}.{}", slugify(prompt), ts, ext))
}

pub async fn download_file(
    client: &reqwest::Client,
    url: &str,
    dest: &Path,
) -> Result<(), GenaiError> {
    let resp = client
        .get(url)
        .send()
        .await
        .map_err(|e| GenaiError::Transient(format!("Download failed: {e}")))?;

    if !resp.status().is_success() {
        return Err(GenaiError::Transient(format!(
            "Download returned HTTP {}",
            resp.status()
        )));
    }

    let bytes = resp
        .bytes()
        .await
        .map_err(|e| GenaiError::Transient(format!("Failed to read download body: {e}")))?;

    std::fs::write(dest, &bytes)
        .map_err(|e| GenaiError::Transient(format!("Failed to write {}: {e}", dest.display())))?;

    Ok(())
}

pub fn print_status(msg: &str) {
    eprint!("\r\x1b[2K  {msg}");
}

pub fn print_success(output: &MediaOutput) {
    let secs = output.elapsed.as_secs_f32();
    eprintln!(
        "\r\x1b[2K  {} Saved to {} ({:.1}s, {}/{})",
        "✓".if_supports_color(Stdout, |t| t.green()),
        output.local_path.display(),
        secs,
        output.provider,
        output.model,
    );
}

#[derive(Serialize)]
pub struct SuccessJson {
    pub status: &'static str,
    pub file: String,
    pub provider: String,
    pub model: String,
    pub elapsed_ms: u64,
    pub prompt: String,
}

#[derive(Serialize)]
pub struct ErrorJson {
    pub status: &'static str,
    pub code: i32,
    pub error: String,
    pub suggestion: String,
}

pub fn print_success_json(output: &MediaOutput, prompt: &str) {
    let json = SuccessJson {
        status: "ok",
        file: output.local_path.display().to_string(),
        provider: output.provider.clone(),
        model: output.model.clone(),
        elapsed_ms: output.elapsed.as_millis() as u64,
        prompt: prompt.to_string(),
    };
    println!("{}", serde_json::to_string_pretty(&json).unwrap());
}

pub fn print_error_json(err: &GenaiError) {
    let json = ErrorJson {
        status: "error",
        code: err.exit_code(),
        error: err.to_string(),
        suggestion: err.suggestion().to_string(),
    };
    println!("{}", serde_json::to_string_pretty(&json).unwrap());
}
