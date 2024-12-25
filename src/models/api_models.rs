use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseData {
    pub license: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RenewData {
    pub license: String,
    pub days: i64
}