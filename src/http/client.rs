use std::{path::Path, sync::Arc};

use reqwest::{Method, Response};
use serde::de::DeserializeOwned;

use crate::{
    LsResult,
    api::{Products, Sales},
    auth::{Config, Tokens},
    http::Authenticator,
};

pub struct LightspeedClient {
    pub(crate) inner: Arc<LightspeedClientInner>,
}

#[derive(Clone)]
pub(crate) struct LightspeedClientInner {
    http: reqwest::Client,
    auth: Arc<Authenticator>,
}

impl LightspeedClient {
    pub async fn from_path<P: AsRef<Path> + Into<String> + std::fmt::Debug>(
        config: Config,
        token_path: P,
    ) -> LsResult<Self> {
        let auth = Authenticator::load(token_path, &config, reqwest::Client::new()).await?;

        Ok(Self::from_authenticator(auth))
    }

    pub async fn from_config(config: Config) -> LsResult<Self> {
        let auth = Authenticator::load(
            Tokens::path_from_domain(&config.domain_prefix),
            &config,
            reqwest::Client::new(),
        )
        .await?;

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

    pub fn products(&self) -> Products {
        Products {
            client: self.inner.clone(),
        }
    }

    pub fn sales(&self) -> Sales {
        Sales {
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
        let body = response.text().await?;
        std::fs::write(
            format!("cache/api/{}.json", str::replace(endpoint, "/", "_")),
            &body,
        )?;
        let value: T = serde_json::from_str(&body)?;

        Ok(value)
    }

    async fn request(&self, method: Method, endpoint: &str) -> LsResult<Response> {
        let token = self.auth.bearer_token().await?;

        let url = format!("{}{}", self.auth.base_url(), endpoint);

        println!("{} {}", method, url);

        let response = self
            .http
            .request(method, url)
            .bearer_auth(token)
            .header("accept", "application/json")
            .send()
            .await?;

        Ok(response)
    }
}
