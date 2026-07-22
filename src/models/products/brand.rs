use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Brand {
    pub id: String,
    pub name: String,
    pub version: i64,
    pub deleted_at: Option<String>,
}
