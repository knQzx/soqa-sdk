use thiserror::Error;

#[derive(Error, Debug)]
pub enum SoqaError {
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Exchange not supported: {0}")]
    ExchangeNotSupported(String),
    #[error("Export error: {0}")]
    ExportError(String),
    #[error("Connection error: {0}")]
    ConnectionError(String),
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
}