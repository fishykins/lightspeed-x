use serde::{Deserialize, Serialize};

use crate::models::sales::{Customization, Pricing, Promotion, TaxComponent};

#[derive(Debug, Deserialize, Serialize)]
pub struct LineItem {
    pub id: String,

    pub quantity: Option<f64>,

    pub status: Option<String>,

    pub note: Option<String>,

    pub gift_card_number: Option<String>,

    #[serde(rename = "_metadata")]
    pub metadata: LineItemMetadata,

    pub pricing: Option<Pricing>,

    pub product: Option<ProductReference>,

    pub customizations: Vec<Customization>,

    pub promotions: Vec<Promotion>,

    pub surcharges: Vec<LineItemSurcharge>,

    #[serde(rename = "return")]
    pub return_info: Option<LineItemReturn>,

    pub tax: Option<super::Tax>,

    pub salesperson_id: Option<String>,

    pub source: Option<LineItemSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineItemMetadata {
    pub is_price_override: bool,

    pub sequence: u32,

    pub tax_components: Vec<TaxComponent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductReference {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineItemReturn {
    pub is_return: bool,

    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineItemSurcharge {
    pub value: f64,

    pub tax_components: Vec<TaxComponent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineItemSource {
    #[serde(default)]
    pub id: Option<String>,
}
