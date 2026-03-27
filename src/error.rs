use thiserror::Error;

#[derive(Debug, Error)]
pub enum GenaiError {
    #[error("{0}")]
    Transient(String),

    #[error("{0}")]
    Config(String),

    #[error("{0}")]
    BadInput(String),

    #[error("{0}")]
    RateLimited(String),
}

impl GenaiError {
    pub fn exit_code(&self) -> i32 {
        match self {
            GenaiError::Transient(_) => 1,
            GenaiError::Config(_) => 2,
            GenaiError::BadInput(_) => 3,
            GenaiError::RateLimited(_) => 4,
        }
    }

    pub fn suggestion(&self) -> &str {
        match self {
            GenaiError::Transient(_) => "Retry the request — this may be a temporary issue",
            GenaiError::Config(_) => "Set FAL_KEY or OPENAI_API_KEY. Get a fal.ai key at https://fal.ai/dashboard/keys",
            GenaiError::BadInput(_) => "Check your prompt and options",
            GenaiError::RateLimited(_) => "Wait 60 seconds or switch provider with --provider openai",
        }
    }
}
