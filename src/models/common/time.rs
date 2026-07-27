use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer};

pub fn deserialize_lightspeed_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;

    // RFC3339 (2026-07-27T10:03:38Z)
    if let Ok(dt) = DateTime::parse_from_rfc3339(&value) {
        return Ok(dt.with_timezone(&Utc));
    }

    // SQL style (2026-07-27 10:03:38)
    if let Ok(dt) = NaiveDateTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S") {
        return Ok(Utc.from_utc_datetime(&dt));
    }

    Err(serde::de::Error::custom(format!(
        "unsupported Lightspeed datetime format: {}",
        value
    )))
}

pub fn deserialize_optional_lightspeed_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<String>::deserialize(deserializer)?;

    let Some(value) = value else {
        return Ok(None);
    };

    // RFC3339
    if let Ok(dt) = DateTime::parse_from_rfc3339(&value) {
        return Ok(Some(dt.with_timezone(&Utc)));
    }

    // SQL style
    if let Ok(dt) = NaiveDateTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S") {
        return Ok(Some(Utc.from_utc_datetime(&dt)));
    }

    Err(serde::de::Error::custom(format!(
        "unsupported Lightspeed datetime format: {}",
        value
    )))
}
