use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Hash)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub version: u64,
}
