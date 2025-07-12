use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Imposter {
    path: String,
    pub default_response: Option<DefaultResponse>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImposterPayload {
    pub port: u16,
    pub protocol: String,
    pub default_response: Option<DefaultResponse>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DefaultResponse {
    pub status_code: u16,
    pub body: String,
    pub headers: HashMap<String, String>,
}

impl Imposter {
    pub fn new(path: impl Into<String>, default_response: Option<DefaultResponse>) -> Self {
        Self {
            path: path.into(),
            default_response,
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}
