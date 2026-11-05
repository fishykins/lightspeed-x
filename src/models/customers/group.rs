use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerGroup {
    #[serde(deserialize_with = "crate::models::common::deserialize_lightspeed_datetime")]
    pub created_at: DateTime<Utc>,
    /// Some lightspeed half string, half id. Not sure what it is used for but not overly
    /// important... probably.
    pub group_id: String,
    /// Unique id, pk.
    pub id: Uuid,
    pub name: String,
    #[serde(deserialize_with = "crate::models::common::deserialize_lightspeed_datetime")]
    pub updated_at: DateTime<Utc>,
    pub version: u64,
}
