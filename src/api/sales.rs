use crate::{
    LsResult,
    api::get_listed_item,
    http::{LightspeedClientInner, Request},
    models::{
        common::{ListResponse, ObjectResponse},
        sales::Sale,
    },
};
use std::sync::Arc;

const MAX_CALLS: usize = 32;
const PAGE_SIZE: usize = 1000;

pub struct Sales {
    pub(crate) client: Arc<LightspeedClientInner>,
}

impl Sales {
    pub async fn get(&self, uuid: &str) -> LsResult<ObjectResponse<Sale>> {
        let request = Request::get(format!("sales/{}", uuid));
        self.client.request(request).await
    }

    pub async fn get_all(&self) -> LsResult<ListResponse<Sale>> {
        get_listed_item(self.client.clone(), "sales", PAGE_SIZE, MAX_CALLS).await
    }
}
