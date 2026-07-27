mod id;
mod response;
mod time;
mod version;

pub use id::IdReference;
pub use response::{ListResponse, ObjectResponse};
use serde::{Deserialize, Deserializer};
pub use time::{deserialize_lightspeed_datetime, deserialize_optional_lightspeed_datetime};
pub use version::VersionRange;

pub fn null_bool_is_false<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Option::<bool>::deserialize(deserializer)?.unwrap_or(false))
}
