use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::{
    common::IdReference,
    sales::{LineItem, Payment, SaleAdjustment, TaxSummary, payment::ExternalApplication},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Sale {
    pub id: String,

    #[serde(rename = "_metadata")]
    pub metadata: SaleMetadata,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,

    pub deleted_at: Option<DateTime<Utc>>,

    pub date: DateTime<Utc>,

    pub state: SaleState,

    pub customer_id: Option<String>,

    pub invoice_number: Option<String>,

    pub receipt_number: Option<String>,

    pub short_code: Option<String>,

    pub note: Option<String>,

    pub attributes: Vec<String>,

    pub adjustments: Vec<SaleAdjustment>,

    //pub ecom_custom_charges: Vec<EcommerceCharge>,
    pub line_items: Vec<LineItem>,

    pub payments: Vec<Payment>,

    pub taxes: Vec<TaxSummary>,

    pub totals: Option<SaleTotals>,

    pub source: Option<SaleSource>,

    #[serde(rename = "return")]
    pub return_info: Option<SaleReturn>,

    pub external_applications: Vec<ExternalApplication>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleTotals {
    pub loyalty: f64,

    pub price: f64,

    pub price_incl_tax: f64,

    pub surcharge: f64,

    pub tax: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleReturn {
    pub is_return: bool,

    pub original_sale_id: Option<String>,

    pub return_sale_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleMetadata {
    pub complete_open_sequence_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::models::common::null_bool_is_false"
    )]
    pub has_unsynced_on_account_payments: bool,

    pub version: u64,

    pub xero_reference: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleSource {
    #[serde(default)]
    pub id: Option<String>,

    #[serde(default)]
    pub outlet_id: Option<String>,

    #[serde(default)]
    pub register_id: Option<String>,

    #[serde(rename = "type")]
    pub source_type: String,

    pub author: Option<IdReference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SaleState {
    Open,
    Closed,
    Parked,
    Voided,
    Completed,

    #[serde(untagged)]
    Other(String),
}
