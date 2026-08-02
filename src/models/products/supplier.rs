use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Supplier {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub source: Option<String>,
    pub version: Option<u64>,
}
