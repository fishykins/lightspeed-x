use crate::{
    LsResult,
    http::LightspeedClientInner,
    models::products::{ProductList, ProductResponse},
};
use std::sync::Arc;

pub struct Products {
    pub(crate) client: Arc<LightspeedClientInner>,
}

impl Products {
    pub async fn get(&self, uuid: &str) -> LsResult<ProductResponse> {
        self.client.get(&format!("products/{}", uuid)).await
    }

    pub async fn get_all(&self) -> LsResult<ProductList> {
        self.client.get("products").await
    }
}
