use crate::{
    LsResult,
    http::{LightspeedClientInner, Request},
    models::{
        common::{ListResponse, ObjectResponse},
        products::Product,
    },
};
use std::sync::Arc;

const MAX_CALLS: u32 = 6;

pub struct Products {
    pub(crate) client: Arc<LightspeedClientInner>,
}

impl Products {
    pub async fn get(&self, uuid: &str) -> LsResult<ObjectResponse<Product>> {
        let request = Request::get(format!("products/{}", uuid));
        self.client.request(request).await
    }

    pub async fn get_all(&self) -> LsResult<ListResponse<Product>> {
        let request = Request::get("products").with_query("page_size", "1000");
        let mut response: ListResponse<Product> = self.client.request(request).await?;

        let mut calls = MAX_CALLS;
        let mut response_len = response.data.len();

        while response_len == 1000 && calls > 0 {
            println!(
                "Getting additional page of products after {}...",
                response.version.max.to_string()
            );
            let request = Request::get("products")
                .with_query("page_size", "1000")
                .with_query("after", response.version.max.to_string());

            let next: ListResponse<Product> = self.client.request(request.clone()).await?;

            if next.data.is_empty() {
                break;
            }

            response_len = next.data.len();
            response.data.extend(next.data);
            response.version.max = next.version.max;
            calls -= 1;
        }

        if calls == 0 {
            println!("Not enough api calls allowed- consider upping MAX_CALLS const")
        }

        Ok(response)
    }
}
