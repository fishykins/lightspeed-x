pub mod auth;
pub mod client;

#[derive(Debug, thiserror::Error)]
pub enum LsError {
    #[error("OAuth error: {0}")]
    OAuth(String),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),

    #[error("Environment variable error: {0}")]
    Env(#[from] std::env::VarError),

    #[error("{0}")]
    Other(String),
}

pub type LsResult<T> = std::result::Result<T, LsError>;
