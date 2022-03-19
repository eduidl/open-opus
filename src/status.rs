use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "success")]
pub enum Status {
    #[serde(rename = "true")]
    Ok(OkStatus),
    #[serde(rename = "false")]
    Err(ErrStatus),
}

#[derive(Debug, Clone, Deserialize)]
pub struct OkStatus {
    pub version: String, // common
    pub source: String,
    pub rows: u64,
    #[serde(rename = "processingtime")]
    pub processing_time: f64, // common
    pub api: String, // common
}

#[derive(Debug, Clone, Deserialize)]
pub struct ErrStatus {
    pub version: String, // common
    pub error: String,
    #[serde(rename = "processingtime")]
    pub processing_time: f64, // common
    pub api: String, // common
}
