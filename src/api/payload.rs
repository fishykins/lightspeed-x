use serde_json::{Map, Value};

use crate::models::products::Product;

pub struct Payload {
    inner: Map<String, Value>,
}

impl Payload {
    pub fn new() -> Self {
        Self { inner: Map::new() }
    }

    pub fn insert<T: serde::Serialize>(&mut self, key: impl Into<String>, value: T) {
        self.inner
            .insert(key.into(), serde_json::to_value(value).unwrap());
    }

    pub fn insert_option<T: serde::Serialize>(&mut self, key: impl Into<String>, value: Option<T>) {
        if let Some(value) = value {
            self.insert(key, value);
        }
    }

    pub fn build(self) -> Value {
        Value::Object(self.inner)
    }
}

impl From<&Product> for Payload {
    fn from(product: &Product) -> Self {
        let mut p = Payload::new();

        p.insert("name", product.name.clone());
        p.insert("sku", product.sku.clone());
        p.insert("is_active", product.is_active);
        p.insert_option("description", product.description.clone());
        p.insert_option("brand_id", product.brand_id.clone());
        p.insert_option("price_including_tax", product.price_including_tax);
        p.insert_option("supply_price", product.supply_price);
        //p.insert_option("procuct_category_id", )

        if let Some(supplier) = &product.supplier {
            p.insert("supplier_id", supplier.id.clone());
        }

        p
    }
}
