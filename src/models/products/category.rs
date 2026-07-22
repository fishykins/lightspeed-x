use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub version: u64,
}
