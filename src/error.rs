use thiserror::Error;

#[derive(Debug, Error)]
pub enum OpenOpusError {
    #[error("HTTP Error {source:?}")]
    ReqwestError {
        #[from]
        source: reqwest::Error,
    },
    #[error("Open Opus API Error {0}")]
    OpenOpusAPIError(String),
}

pub type OpenOpusResult<T> = Result<T, OpenOpusError>;
