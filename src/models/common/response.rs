use std::path::PathBuf;

use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{LsResult, models::common::VersionRange};

#[derive(Debug, Deserialize, Serialize)]
pub struct ObjectResponse<T> {
    pub data: T,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListResponse<T> {
    pub data: Vec<T>,
    pub version: VersionRange,
}

impl<T> ListResponse<T>
where
    T: Serialize,
{
    pub async fn save_to_file<P: Into<PathBuf>>(&self, path: P) -> LsResult<()> {
        let json = serde_json::to_vec_pretty(self)?;

        tokio::fs::write(path.into(), json).await?;

        Ok(())
    }
}

impl<T> ListResponse<T>
where
    T: DeserializeOwned,
{
    pub async fn read_from_file<P: Into<PathBuf>>(path: P) -> LsResult<Self> {
        let json = tokio::fs::read(path.into()).await?;

        Ok(serde_json::from_slice(&json)?)
    }
}

impl<T> ObjectResponse<T>
where
    T: Serialize,
{
    pub async fn save_to_file<P: Into<PathBuf>>(&self, path: P) -> LsResult<()> {
        let json = serde_json::to_vec_pretty(self)?;

        tokio::fs::write(path.into(), json).await?;

        Ok(())
    }
}
