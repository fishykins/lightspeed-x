use crate::{
    LsResult,
    http::{LightspeedClientInner, Request},
    models::{common::ListResponse, customers::CustomerGroup},
};
use std::sync::Arc;

pub struct Groups {
    pub(crate) client: Arc<LightspeedClientInner>,
}

impl Groups {
    pub async fn get_all(&self) -> LsResult<ListResponse<CustomerGroup>> {
        let request = Request::get("customer_groups").with_query("page_size", "1000");
        self.client.request(request).await
    }
}
