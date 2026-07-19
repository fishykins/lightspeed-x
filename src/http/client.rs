use std::{path::Path, sync::Arc};

use reqwest::{Method, Response};
use serde::de::DeserializeOwned;

use crate::{LsResult, api::Products, auth::Config, http::Authenticator};

pub struct LightspeedClient {
    pub(crate) inner: Arc<LightspeedClientInner>,
}

#[derive(Clone)]
pub(crate) struct LightspeedClientInner {
    http: reqwest::Client,
    auth: Arc<Authenticator>,
}

impl LightspeedClient {
    pub async fn new<P: AsRef<Path> + Into<String>>(
        config: Config,
        token_path: P,
    ) -> LsResult<Self> {
        let auth = Authenticator::load(token_path, &config, reqwest::Client::new()).await?;

        Ok(Self::from_authenticator(auth))
    }

    pub fn from_authenticator(auth: Authenticator) -> Self {
        Self {
            inner: Arc::new(LightspeedClientInner {
                http: auth.http.clone(),
                auth: Arc::new(auth),
            }),
        }
    }

    pub async fn blast_api_with_nonsense(&self) -> LsResult<()> {
        let token = self.inner.auth.bearer_token().await?;

        let url = format!(
            "{}sales/65ba0a2f-b33b-828d-11f1-80f7ddf85842",
            self.inner.auth.base_url()
        );

        println!("GET {}", url);

        let response = self.inner.http.get(url).bearer_auth(token).send().await?;
        let body = response.text().await?;

        println!("Response body:");
        println!("{}", body);

        Ok(())
    }

    pub fn products(&self) -> Products {
        Products {
            client: self.inner.clone(),
        }
    }
}

impl LightspeedClientInner {
    pub(crate) async fn get<T>(&self, endpoint: &str) -> LsResult<T>
    where
        T: DeserializeOwned,
    {
        let response = self.request(Method::GET, endpoint).await?;

        println!("Response: {:?}", response);

        Ok(response.json::<T>().await?)
    }

    async fn request(&self, method: Method, endpoint: &str) -> LsResult<Response> {
        let token = self.auth.bearer_token().await?;

        let url = format!("{}{}", self.auth.base_url(), endpoint);

        println!("{} {}", method, url);

        let response = self
            .http
            .request(method, url)
            .bearer_auth(token)
            .send()
            .await?;

        Ok(response)
    }
}
