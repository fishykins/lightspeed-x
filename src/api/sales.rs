use crate::{
    LsResult,
    http::LightspeedClientInner,
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
        self.client.get(&format!("sales/{}", uuid)).await
    }

    pub async fn get_all(&self) -> LsResult<ListResponse<Sale>> {
        self.client.get("sales").await
    }
}
