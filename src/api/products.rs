use crate::{
    LsResult,
    http::LightspeedClientInner,
    models::{
        common::{ListResponse, ObjectResponse},
        products::Product,
    },
};
use std::sync::Arc;

pub struct Products {
    pub(crate) client: Arc<LightspeedClientInner>,
}

impl Products {
    pub async fn get(&self, uuid: &str) -> LsResult<ObjectResponse<Product>> {
        self.client.get(&format!("products/{}", uuid)).await
    }

    pub async fn get_all(&self) -> LsResult<ListResponse<Product>> {
        self.client.get("products").await
    }
}
