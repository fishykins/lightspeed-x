use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Taken from the payload of Sales webhook- when doing customer webhook check this is the same!
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookCustomer {
    pub balance: Decimal,

    pub company_name: Option<String>,

    pub contact_first_name: Option<String>,
    pub contact_last_name: Option<String>,

    pub created_at: DateTime<Utc>,

    pub custom_field_1: Option<String>,
    pub custom_field_2: Option<String>,
    pub custom_field_3: Option<String>,
    pub custom_field_4: Option<String>,

    pub customer_code: String,

    pub customer_group_id: Uuid,

    pub date_of_birth: Option<String>,

    pub deleted_at: Option<DateTime<Utc>>,

    pub do_not_email: bool,

    pub email: Option<String>,

    pub enable_loyalty: bool,

    pub fax: Option<String>,

    pub first_name: Option<String>,

    pub id: Uuid,

    pub last_name: Option<String>,

    pub loyalty_balance: Decimal,

    pub mobile: Option<String>,

    pub note: Option<String>,

    pub phone: Option<String>,

    pub sex: Option<String>,

    pub updated_at: DateTime<Utc>,

    pub year_to_date: Decimal,
}
