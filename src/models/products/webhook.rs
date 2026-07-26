use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookProduct {
    pub active: bool,

    pub attributed_cost: Option<Decimal>,

    pub base_name: String,

    pub brand: WebhookBrand,

    pub button_order: Option<i32>,

    pub categories: Vec<WebhookCategory>,

    pub deleted_at: Option<String>,

    pub description: String,

    pub handle: String,

    pub id: Uuid,

    pub inventory: Vec<WebhookInventory>,

    pub name: String,

    pub price_book_entries: Vec<WebhookPriceBookEntry>,

    pub product_type: WebhookProductType,

    pub retailer_id: Uuid,

    pub sku: String,

    pub source: Option<String>,

    pub source_id: Option<String>,

    pub source_variant_id: Option<String>,

    pub supplier: Option<WebhookSupplier>,

    pub supply_price: Decimal,

    pub taxes: Vec<WebhookProductTax>,

    pub variant_options: Vec<WebhookVariantOption>,

    pub variant_parent_id: Option<Uuid>,

    pub version: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookBrand {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookCategory {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookInventory {
    pub attributed_cost: Decimal,

    pub count: Decimal,

    pub id: Uuid,

    pub outlet_id: Uuid,

    pub product_id: Uuid,

    pub reorder_point: Decimal,

    pub restock_level: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookPriceBookEntry {
    pub customer_group_id: Uuid,

    pub customer_group_name: String,

    pub id: Uuid,

    pub max_units: Option<Decimal>,

    pub min_units: Option<Decimal>,

    pub price: Decimal,

    pub product_id: Uuid,

    #[serde(rename = "type")]
    pub price_type: String,

    pub valid_from: Option<String>,

    pub valid_to: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookProductType {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookSupplier {
    pub description: String,

    pub id: Uuid,

    pub name: String,

    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookProductTax {
    pub outlet_id: Uuid,

    pub tax_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookVariantOption {
    pub id: Uuid,

    pub name: String,

    pub value: String,
}
