pub mod api;
pub mod auth;
pub mod http;
pub mod models;
pub mod webhooks;

#[derive(Debug, thiserror::Error)]
pub enum LsError {
    // ===== OAuth ===========================================================
    #[error("OAuth error: {0}")]
    OAuth(String),

    // ===== HTTP ============================================================
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("HTTP header error: {0}")]
    Header(#[from] reqwest::header::ToStrError),

    // ===== Parsing =========================================================
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Failed to decode webhook form: {0}")]
    Form(#[from] serde_urlencoded::de::Error),

    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),

    #[error("UUID parse error: {0}")]
    Uuid(#[from] uuid::Error),

    #[error("Date/time parse error: {0}")]
    Chrono(#[from] chrono::ParseError),

    #[error("Decimal parse error: {0}")]
    Decimal(#[from] rust_decimal::Error),

    // ===== Filesystem ======================================================
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Environment variable error: {0}")]
    Env(#[from] std::env::VarError),

    // ===== Webhooks ========================================================
    #[error("Webhook request is missing a payload field")]
    MissingPayload,

    #[error("Webhook request is missing the X-Signature header")]
    MissingSignature,

    #[error("Invalid X-Signature header")]
    InvalidSignatureHeader,

    #[error("Unsupported signature algorithm: {0}")]
    UnsupportedSignatureAlgorithm(String),

    #[error("Webhook signature verification failed")]
    InvalidSignature,

    // ===== API =============================================================
    #[error("Unexpected API response: {0}")]
    UnexpectedResponse(String),

    #[error("Missing required field: {0}")]
    MissingField(&'static str),

    #[error("{0}")]
    Other(String),
}

pub type LsResult<T> = std::result::Result<T, LsError>;

pub const LS_VERSION: &str = "2026-07/";
