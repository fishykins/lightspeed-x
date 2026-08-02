use crate::{
    LsResult,
    http::{LightspeedClientInner, Request},
    models::{
        common::{ListResponse, ObjectResponse},
        sales::Sale,
    },
};
use std::sync::Arc;

pub struct Sales {
    pub(crate) client: Arc<LightspeedClientInner>,
}

impl Sales {
    pub async fn get(&self, uuid: &str) -> LsResult<ObjectResponse<Sale>> {
        let request = Request::get(format!("sales/{}", uuid));
        self.client.request(request).await
    }

    pub async fn get_all(&self) -> LsResult<ListResponse<Sale>> {
        let request = Request::get("sales");
        self.client.request(request).await
    }
}
