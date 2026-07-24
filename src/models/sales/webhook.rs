use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::customers::WebhookCustomer;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookSale {
    pub created_at: NaiveDateTime,

    pub customer: WebhookCustomer,
    pub customer_id: Uuid,

    pub deleted_at: Option<NaiveDateTime>,

    pub id: Uuid,

    pub invoice_number: String,

    pub note: String,

    pub register_id: Uuid,

    pub register_sale_payments: Vec<WebhookSalePayment>,

    pub register_sale_products: Vec<WebhookSaleProduct>,

    pub sale_date: DateTime<Utc>,

    pub short_code: String,

    pub source: String,

    pub source_id: Option<String>,

    pub status: String,

    pub taxes: Vec<WebhookSaleTax>,

    pub totals: WebhookSaleTotals,

    pub updated_at: DateTime<FixedOffset>,

    pub user: WebhookUser,

    pub user_id: Uuid,

    pub version: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookUser {
    pub created_at: NaiveDateTime,

    pub display_name: String,

    pub email: String,

    pub id: Uuid,

    pub name: String,

    pub target_daily: Option<Decimal>,
    pub target_weekly: Option<Decimal>,
    pub target_monthly: Option<Decimal>,

    pub updated_at: NaiveDateTime,
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
