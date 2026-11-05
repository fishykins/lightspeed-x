use uuid::Uuid;

use crate::{
    LsResult,
    api::{Payload, get_listed_item},
    http::{LightspeedClientInner, Request},
    models::{
        common::{ListResponse, ObjectResponse},
        products::Product,
    },
};
use std::sync::Arc;

const MAX_CALLS: usize = 6;
const PAGE_SIZE: usize = 1000;

pub struct Products {
    pub(crate) client: Arc<LightspeedClientInner>,
}

impl Products {
    pub async fn get(&self, uuid: &str) -> LsResult<ObjectResponse<Product>> {
        let request = Request::get(format!("products/{}", uuid));
        self.client.request(request).await
    }

    pub async fn get_all(&self) -> LsResult<ListResponse<Product>> {
        get_listed_item(self.client.clone(), "products", PAGE_SIZE, MAX_CALLS).await
    }

    pub async fn upload(&self, product: &Product) -> LsResult<Vec<Uuid>> {
        let payload = Payload::from(product).build();

        let request = Request::post("product").with_payload(payload);

        let response = self.client.request(request).await?;

        Ok(response)
    }
}
