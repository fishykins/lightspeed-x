use std::sync::Arc;

use crate::{LsResult, http::LightspeedClientInner};

pub struct Products {
    pub(crate) client: Arc<LightspeedClientInner>,
}

impl Products {
    pub async fn get(&self, uuid: &str) -> LsResult<String> {
        self.client.get(&format!("products/{}", uuid)).await
    }

    pub async fn get_all(&self) -> LsResult<String> {
        self.client.get("products").await
    }
}
