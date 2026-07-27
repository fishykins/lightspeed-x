use std::collections::HashMap;

use serde::{Deserialize, de::DeserializeOwned};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct WebhookForm {
    pub payload: String,
    #[serde(default)]
    pub domain_prefix: Option<String>,
    #[serde(default)]
    pub environment: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebhookKind {
    Sale,
    Product,
    Customer,
    Inventory,
    Unknown,
}

impl WebhookForm {
    pub fn parse<T>(&self) -> Result<T, serde_json::Error>
    where
        T: DeserializeOwned,
    {
        serde_json::from_str(&self.payload)
    }

    pub fn kind(&self) -> WebhookKind {
        let Ok(value) = serde_json::from_str::<Value>(&self.payload) else {
            return WebhookKind::Unknown;
        };

        if looks_like_sale(&value) {
            return WebhookKind::Sale;
        }

        if looks_like_product(&value) {
            return WebhookKind::Product;
        }

        if looks_like_customer(&value) {
            return WebhookKind::Customer;
        }

        WebhookKind::Unknown
    }
}

fn looks_like_product(value: &Value) -> bool {
    let Some(obj) = value.as_object() else {
        return false;
    };

    obj.contains_key("base_name") && obj.contains_key("sku") && obj.contains_key("brand")
}

fn looks_like_sale(value: &Value) -> bool {
    let Some(obj) = value.as_object() else {
        return false;
    };

    obj.contains_key("sale_date") && obj.contains_key("taxes") && obj.contains_key("register_id")
}

fn looks_like_customer(value: &Value) -> bool {
    let Some(obj) = value.as_object() else {
        return false;
    };

    obj.contains_key("balance") && obj.contains_key("contact_first_name")
}
