use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookSale {
    #[serde(deserialize_with = "crate::models::common::deserialize_lightspeed_datetime")]
    pub created_at: DateTime<Utc>,

    pub customer: Option<WebhookSaleCustomer>,
    pub customer_id: Option<Uuid>,

    #[serde(deserialize_with = "crate::models::common::deserialize_optional_lightspeed_datetime")]
    pub deleted_at: Option<DateTime<Utc>>,

    pub id: Uuid,

    pub invoice_number: String,

    pub note: Option<String>,

    pub register_id: Uuid,

    pub register_sale_payments: Vec<WebhookSalePayment>,

    pub register_sale_products: Vec<WebhookSaleProduct>,

    #[serde(deserialize_with = "crate::models::common::deserialize_lightspeed_datetime")]
    pub sale_date: DateTime<Utc>,

    pub short_code: String,

    pub source: String,

    pub source_id: Option<String>,

    pub status: String,

    pub taxes: Vec<WebhookSaleTax>,

    pub totals: WebhookSaleTotals,

    #[serde(deserialize_with = "crate::models::common::deserialize_lightspeed_datetime")]
    pub updated_at: DateTime<Utc>,

    pub user: WebhookUser,

    pub user_id: Uuid,

    pub version: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookSaleCustomer {
    pub balance: Decimal,

    pub company_name: Option<String>,

    pub contact_first_name: Option<String>,
    pub contact_last_name: Option<String>,

    #[serde(deserialize_with = "crate::models::common::deserialize_lightspeed_datetime")]
    pub created_at: DateTime<Utc>,

    pub custom_field_1: Option<String>,
    pub custom_field_2: Option<String>,
    pub custom_field_3: Option<String>,
    pub custom_field_4: Option<String>,

    pub customer_code: String,

    pub customer_group_id: Uuid,

    pub date_of_birth: Option<String>,

    #[serde(
        default,
        deserialize_with = "crate::models::common::deserialize_optional_lightspeed_datetime"
    )]
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

    pub points: i64,

    pub sex: Option<String>,

    #[serde(deserialize_with = "crate::models::common::deserialize_lightspeed_datetime")]
    pub updated_at: DateTime<Utc>,

    pub year_to_date: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookUser {
    #[serde(deserialize_with = "crate::models::common::deserialize_lightspeed_datetime")]
    pub created_at: DateTime<Utc>,

    pub display_name: String,

    pub email: Option<String>,

    pub id: Uuid,

    pub name: String,

    pub target_daily: Option<Decimal>,
    pub target_weekly: Option<Decimal>,
    pub target_monthly: Option<Decimal>,

    #[serde(deserialize_with = "crate::models::common::deserialize_lightspeed_datetime")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookSaleTotals {
    pub total_loyalty: Decimal,

    pub total_payment: Decimal,

    pub total_price: Decimal,

    pub total_tax: Decimal,

    pub total_to_pay: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookSaleTax {
    pub id: Uuid,

    pub name: String,

    pub rate: Decimal,

    pub tax: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookSaleProduct {
    pub discount: Decimal,

    pub id: Uuid,

    pub loyalty_value: Decimal,

    pub price: Decimal,

    pub price_set: bool,

    pub price_total: Decimal,

    pub product_id: Uuid,

    pub quantity: Decimal,

    pub tax: Decimal,

    pub tax_id: Uuid,

    pub tax_total: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookSalePayment {
    pub amount: Decimal,

    pub id: Uuid,

    #[serde(deserialize_with = "crate::models::common::deserialize_lightspeed_datetime")]
    pub payment_date: DateTime<Utc>,

    pub payment_type: WebhookPaymentType,

    pub payment_type_id: i32,

    pub retailer_payment_type: WebhookRetailerPaymentType,

    pub retailer_payment_type_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookPaymentType {
    pub has_native_support: bool,

    pub id: String,

    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookRetailerPaymentType {
    pub config: Option<serde_json::Value>,

    pub id: Uuid,

    pub name: String,

    pub payment_type_id: String,
}
