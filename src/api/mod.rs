mod brands;
mod contact;
mod customers;
mod groups;
mod payload;
mod products;
mod sales;

use std::sync::Arc;

pub use brands::Brands;
pub use customers::Customers;
pub use groups::Groups;
pub use payload::Payload;
pub use products::Products;
pub use sales::Sales;
use serde::de::DeserializeOwned;

use crate::{
    LsResult,
    http::{LightspeedClientInner, Request},
    models::common::ListResponse,
};

pub(crate) async fn get_listed_item<T>(
    client: Arc<LightspeedClientInner>,
    request_string: &str,
    page_size: usize,
    api_calls_max: usize,
) -> LsResult<ListResponse<T>>
where
    T: DeserializeOwned,
{
    let request = Request::get(request_string).with_query("page_size", page_size.to_string());
    let mut response: ListResponse<T> = client.request(request).await?;

    let mut calls = api_calls_max;
    let mut response_len = response.data.len();

    while response_len == page_size && calls > 0 {
        println!(
            "Getting additional page of products after {}...",
            response.version.max.to_string()
        );
        let request = Request::get(request_string)
            .with_query("page_size", page_size.to_string())
            .with_query("after", response.version.max.to_string());

        let next: ListResponse<T> = client.request(request.clone()).await?;

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
