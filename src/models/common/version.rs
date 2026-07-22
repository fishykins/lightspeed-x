use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct VersionRange {
    pub min: u64,
    pub max: u64,
}
