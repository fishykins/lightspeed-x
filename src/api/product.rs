use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{LsResult, http::LightspeedClientInner};

pub struct Products {
    pub(crate) client: Arc<LightspeedClientInner>,
}

#[derive(Debug, Deserialize)]
pub struct ProductList {
    pub data: Vec<Product>,
    pub version: VersionRange,
}

#[derive(Debug, Deserialize)]
pub struct VersionRange {
    pub min: u64,
    pub max: u64,
}

#[derive(Debug, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub sku: String,

    pub active: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub version: u64,
}

impl Products {
    pub async fn get(&self, uuid: &str) -> LsResult<String> {
        self.client.get(&format!("products/{}", uuid)).await
    }

    pub async fn get_all(&self) -> LsResult<ProductList> {
        self.client.get("products").await
    }
}
