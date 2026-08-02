use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Brand {
    pub id: Uuid,
    pub name: String,
    pub version: i64,
    pub deleted_at: Option<String>,
}
