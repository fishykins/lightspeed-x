use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub balance: Decimal,
    pub company_name: Option<String>,

    #[serde(deserialize_with = "crate::models::common::deserialize_lightspeed_datetime")]
    pub created_at: DateTime<Utc>,

    pub custom_field_1: Option<String>,
    pub custom_field_2: Option<String>,
    pub custom_field_3: Option<String>,
    pub custom_field_4: Option<String>,

    pub customer_code: Option<String>,
    pub customer_group_id: Option<Uuid>,
    pub date_of_birth: Option<NaiveDate>,

    pub do_not_email: bool,
    pub email: Option<String>,
    pub enable_loyalty: bool,
    pub enable_promotional_sms: bool,

    pub fax: Option<String>,
    pub first_name: Option<String>,
    pub gender: Option<String>,

    pub id: Uuid,
    pub last_name: Option<String>,
    pub loyalty_balance: Decimal,
    pub mobile: Option<String>,
    pub name: String,
    pub note: Option<String>,
    pub on_account_limit: Decimal,
    pub phone: Option<String>,

    pub physical_address_1: Option<String>,
    pub physical_address_2: Option<String>,
    pub physical_city: Option<String>,
    pub physical_country_id: Option<String>,
    pub physical_postcode: Option<String>,
    pub physical_state: Option<String>,
    pub physical_suburb: Option<String>,

    pub postal_address_1: Option<String>,
    pub postal_address_2: Option<String>,
    pub postal_city: Option<String>,
    pub postal_country_id: Option<String>,
    pub postal_postcode: Option<String>,
    pub postal_state: Option<String>,
    pub postal_suburb: Option<String>,

    pub twitter: Option<String>,

    #[serde(deserialize_with = "crate::models::common::deserialize_lightspeed_datetime")]
    pub updated_at: DateTime<Utc>,

    pub version: u64,
    pub website: Option<String>,
    pub year_to_date: Decimal,
}
