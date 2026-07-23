use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: String,

    pub amount: f64,

    pub date: DateTime<Utc>,

    pub deleted_at: Option<DateTime<Utc>>,

    pub billing_address_id: Option<String>,

    #[serde(rename = "_metadata")]
    pub metadata: PaymentMetadata,

    pub source: PaymentSource,

    #[serde(rename = "type")]
    pub payment_type: PaymentType,

    pub surcharge: Option<PaymentSurcharge>,

    pub external_applications: Vec<ExternalApplication>,

    pub external_attributes: Vec<ExternalAttribute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMetadata {
    pub register_open_sequence_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentSource {
    pub id: String,

    pub outlet_id: String,

    pub register_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalApplication {
    pub application_id: String,

    pub external_id: String,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,

    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentType {
    pub id: String,

    pub name: String,

    pub config_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentSurcharge {
    pub amount: f64,

    pub rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalAttribute {
    pub source: String,

    pub transaction_id: String,

    pub card_last_four_digits: Option<String>,

    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}
