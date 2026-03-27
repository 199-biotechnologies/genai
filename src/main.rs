mod agent;
mod error;
mod models;
mod output;
mod providers;

use clap::{Parser, Subcommand};
use error::GenaiError;
use output::Format;
use providers::{fal::FalProvider, openai::OpenAiProvider, ImageRequest, ProviderKind, VideoRequest};

#[derive(Parser)]
#[command(name = "genai", version, about = "AI image and video generation CLI")]
struct Cli {
    #[arg(long, global = true)]
    json: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate an image from a text prompt
    Image {
        prompt: String,
        #[arg(long)]
        model: Option<String>,
        #[arg(long)]
        provider: Option<String>,
        #[arg(long, short)]
        output: Option<String>,
        #[arg(long, default_value = "1024x1024")]
        size: String,
    },
    /// Generate a video from a text prompt
    Video {
        prompt: String,
        #[arg(long)]
        model: Option<String>,
        #[arg(long)]
        provider: Option<String>,
        #[arg(long, short)]
        output: Option<String>,
        #[arg(long, default_value = "5")]
        duration: u8,
        #[arg(long, default_value = "16:9")]
        aspect_ratio: String,
    },
    /// List available AI models
    Models {
        #[arg(long = "type")]
        media_type: Option<String>,
    },
    /// Machine-readable capability manifest
    AgentInfo,
    /// Install skill files for AI agent platforms
    Skill {
        #[command(subcommand)]
        action: SkillAction,
    },
}

#[derive(Subcommand)]
enum SkillAction {
    Install,
    Status,
}

fn parse_size(s: &str) -> Result<(u32, u32), GenaiError> {
    let parts: Vec<&str> = s.split('x').collect();
    if parts.len() != 2 {
        return Err(GenaiError::BadInput(format!(
            "Invalid size '{s}'. Use WxH format like 1024x1024"
        )));
    }
    let w = parts[0]
        .parse()
        .map_err(|_| GenaiError::BadInput(format!("Invalid width in '{s}'")))?;
    let h = parts[1]
        .parse()
        .map_err(|_| GenaiError::BadInput(format!("Invalid height in '{s}'")))?;
    Ok((w, h))
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let fmt = Format::detect(cli.json);
    let client = reqwest::Client::new();

    let result = run(cli.command, &client, fmt).await;

    match result {
        Ok((out, prompt)) => match fmt {
            Format::Json => output::print_success_json(&out, &prompt),
            Format::Tty => output::print_success(&out),
        },
        Err(err) => {
            match fmt {
                Format::Json => output::print_error_json(&err),
                Format::Tty => eprintln!("\r\x1b[2K  x {err}"),
            }
            std::process::exit(err.exit_code());
        }
    }
}

async fn run(
    cmd: Commands,
    client: &reqwest::Client,
    fmt: Format,
) -> Result<(providers::MediaOutput, String), GenaiError> {
    match cmd {
        Commands::Image {
            prompt,
            model,
            provider,
            output: out,
            size,
        } => {
            if prompt.trim().is_empty() {
                return Err(GenaiError::BadInput("Prompt cannot be empty".into()));
            }
            let (w, h) = parse_size(&size)?;
            let kind = providers::select_provider(provider.as_deref())?;
            let pname = match kind {
                ProviderKind::Fal => "fal",
                ProviderKind::OpenAi => "openai",
            };
            let mi = models::resolve_model(model.as_deref(), pname, providers::MediaType::Image)
                .ok_or_else(|| {
                    GenaiError::BadInput(format!(
                        "Unknown model '{}'. Run 'genai models --type image' to see options",
                        model.as_deref().unwrap_or("default")
                    ))
                })?;

            let req = ImageRequest {
                prompt: prompt.clone(),
                model_id: mi.model_id.to_string(),
                width: w,
                height: h,
            };

            let result = match kind {
                ProviderKind::Fal => {
                    FalProvider::from_env()?
                        .generate_image(client, &req, out.as_deref())
                        .await
                }
                ProviderKind::OpenAi => {
                    OpenAiProvider::from_env()?
                        .generate_image(client, &req, out.as_deref())
                        .await
                }
            }?;

            Ok((result, prompt))
        }

        Commands::Video {
            prompt,
            model,
            provider,
            output: out,
            duration,
            aspect_ratio,
        } => {
            if prompt.trim().is_empty() {
                return Err(GenaiError::BadInput("Prompt cannot be empty".into()));
            }
            let kind = providers::select_provider(provider.as_deref())?;
            let pname = match kind {
                ProviderKind::Fal => "fal",
                ProviderKind::OpenAi => "openai",
            };
            let mi = models::resolve_model(model.as_deref(), pname, providers::MediaType::Video)
                .ok_or_else(|| {
                    GenaiError::BadInput(format!(
                        "Unknown model '{}'. Run 'genai models --type video' to see options",
                        model.as_deref().unwrap_or("default")
                    ))
                })?;

            let req = VideoRequest {
                prompt: prompt.clone(),
                model_id: mi.model_id.to_string(),
                aspect_ratio,
                duration_secs: duration,
            };

            let result = match kind {
                ProviderKind::Fal => {
                    FalProvider::from_env()?
                        .generate_video(client, &req, out.as_deref())
                        .await
                }
                ProviderKind::OpenAi => {
                    OpenAiProvider::from_env()?
                        .generate_video(client, &req, out.as_deref())
                        .await
                }
            }?;

            Ok((result, prompt))
        }

        Commands::Models { media_type } => {
            let filter = media_type.as_deref().and_then(|t| match t {
                "image" => Some(providers::MediaType::Image),
                "video" => Some(providers::MediaType::Video),
                _ => None,
            });
            let all = models::all_models();
            let filtered: Vec<_> = all
                .iter()
                .filter(|m| filter.map_or(true, |f| f == m.media_type))
                .collect();

            if matches!(fmt, Format::Json) {
                let json: Vec<_> = filtered
                    .iter()
                    .map(|m| {
                        serde_json::json!({
                            "name": m.name,
                            "type": match m.media_type {
                                providers::MediaType::Image => "image",
                                providers::MediaType::Video => "video",
                            },
                            "provider": m.provider,
                            "description": m.description,
                            "default": m.default,
                        })
                    })
                    .collect();
                println!("{}", serde_json::to_string_pretty(&json).unwrap());
            } else {
                for m in &filtered {
                    let type_str = match m.media_type {
                        providers::MediaType::Image => "image",
                        providers::MediaType::Video => "video",
                    };
                    let def = if m.default { " (default)" } else { "" };
                    println!("  {:16} {:8} {:8} {}{}", m.name, type_str, m.provider, m.description, def);
                }
            }
            std::process::exit(0);
        }

        Commands::AgentInfo => {
            println!("{}", agent::agent_info_json());
            std::process::exit(0);
        }

        Commands::Skill { action } => {
            match action {
                SkillAction::Install => agent::skill_install(),
                SkillAction::Status => agent::skill_status(),
            }
            std::process::exit(0);
        }
    }
}
