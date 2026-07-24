use std::collections::HashMap;

use serde::{Deserialize, de::DeserializeOwned};

#[derive(Debug, Deserialize)]
pub struct WebhookForm {
    pub payload: String,
    #[serde(default)]
    pub domain_prefix: Option<String>,
    #[serde(default)]
    pub environment: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, String>,
}

impl WebhookForm {
    pub fn parse<T>(&self) -> Result<T, serde_json::Error>
    where
        T: DeserializeOwned,
    {
        serde_json::from_str(&self.payload)
    }
}
