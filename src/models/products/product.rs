use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::{
    common::VersionRange,
    products::{Brand, Category, Supplier},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductList {
    pub data: Vec<Product>,
    pub version: VersionRange,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductResponse {
    pub data: Product,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Product {
    // Identity
    pub id: String,
    pub sku: String,
    pub handle: String,

    // Names
    pub name: String,
    pub variant_name: String,

    // Status
    pub active: bool,
    pub is_active: bool,
    pub has_inventory: bool,
    pub has_variants: bool,
    pub is_composite: bool,

    // Pricing
    pub price_excluding_tax: Option<f64>,
    pub price_including_tax: Option<f64>,
    pub supply_price: Option<f64>,

    // Supplier
    pub supplier_code: Option<String>,
    pub supplier_id: Option<String>,

    // Brand
    pub brand_id: Option<String>,

    // Product Type
    pub product_type_id: Option<String>,

    // Ordering
    pub button_order: u32,

    // Misc
    pub description: Option<String>,
    pub source: String,

    // Images
    pub image_thumbnail_url: Option<String>,
    pub image_url: Option<String>,

    // Metadata
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: u64,

    // Relationships
    pub brand: Option<Brand>,
    pub supplier: Option<Supplier>,
    pub categories: Vec<Category>,
    pub images: Vec<ProductImage>,
    pub attributes: Vec<serde_json::Value>,
    pub product_codes: Vec<ProductCode>,
    pub product_suppliers: Vec<Supplier>,
    pub variant_options: Vec<VariantOption>,
    pub tag_ids: Vec<String>,
    #[serde(rename = "type")]
    pub product_type: Option<ProductType>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductType {
    pub id: String,
    pub name: String,
    pub version: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductCode {
    pub code: String,
    #[serde(rename = "type")]
    pub code_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VariantOption {
    pub id: String,
    pub name: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductImage {
    pub id: String,
    pub url: String,
    pub version: u64,
    pub sizes: ImageSizes,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageSizes {
    pub original: String,
    pub standard: String,
    pub thumb: String,

    pub sl: String,
    pub sm: String,
    pub ss: String,
    pub st: String,
}
