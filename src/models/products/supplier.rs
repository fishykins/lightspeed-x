use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Supplier {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub source: Option<String>,
    pub version: Option<u64>,
}
