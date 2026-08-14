use crate::{
    LsResult,
    http::{LightspeedClientInner, Request},
    models::{common::ListResponse, products::Brand},
};
use std::sync::Arc;

pub struct Brands {
    pub(crate) client: Arc<LightspeedClientInner>,
}

impl Brands {
    pub async fn get_all(&self) -> LsResult<ListResponse<Brand>> {
        let request = Request::get("brands").with_query("page_size", "1000");
        self.client.request(request).await
    }
}
