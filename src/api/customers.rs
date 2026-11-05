use uuid::Uuid;

use crate::{
    LsResult,
    api::get_listed_item,
    http::LightspeedClientInner,
    models::{common::ListResponse, customers::Customer},
};
use std::sync::Arc;

const MAX_CALLS: usize = 12;
const PAGE_SIZE: usize = 1000;

pub struct Customers {
    pub(crate) client: Arc<LightspeedClientInner>,
}

impl Customers {
    pub async fn get_all(&self) -> LsResult<ListResponse<Customer>> {
        get_listed_item(self.client.clone(), "customers", PAGE_SIZE, MAX_CALLS).await
    }

    /// Gets all customers who belong to the given group.
    pub async fn get_group(&self, group: Uuid) -> LsResult<ListResponse<Customer>> {
        let endpoint = format!("customer_groups/{}/customers", group.to_string());
        get_listed_item(self.client.clone(), &endpoint, PAGE_SIZE, MAX_CALLS).await
    }
}
