use std::{path::Path, sync::Arc};

use crate::{LsResult, auth::Config, http::Authenticator};

#[derive(Clone)]
pub struct LightspeedClient {
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
            http: auth.http.clone(),
            auth: Arc::new(auth),
        }
    }

    pub async fn blast_api_with_nonsense(&self) -> LsResult<()> {
        let token = self.auth.bearer_token().await?;

        let url = format!(
            "{}sales/65ba0a2f-b33b-828d-11f1-80f7ddf85842",
            self.auth.base_url()
        );

        println!("GET {}", url);

        let response = self.http.get(url).bearer_auth(token).send().await?;
        let body = response.text().await?;

        println!("Response body:");
        println!("{}", body);

        Ok(())
    }
}
