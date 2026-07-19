use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;

use url::Url;

use crate::{
    LsError, LsResult,
    auth::{Config, TokenResponse, Tokens},
};

pub struct Authenticator {
    config: Arc<Config>,
    tokens: Arc<RwLock<Tokens>>,
    token_path: String, // TODO: Make PathBuf
    base_url: Url,
    pub(crate) http: reqwest::Client,
}

impl Authenticator {
    pub async fn bearer_token(&self) -> LsResult<String> {
        {
            let tokens = self.tokens.read().await;

            if !tokens.needs_refresh() {
                return Ok(tokens.access_token.clone());
            }
        }

        self.refresh().await?;
        let tokens = self.tokens.read().await;

        Ok(tokens.access_token.clone())
    }

    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    async fn refresh(&self) -> LsResult<()> {
        // Copy the refresh token so we don't hold the lock during the HTTP request.
        let (refresh_token, domain_prefix) = {
            let tokens = self.tokens.read().await;
            (tokens.refresh_token.clone(), tokens.domain_prefix.clone())
        };

        let response = self
            .http
            .post(format!(
                "https://{}.retail.lightspeed.app/api/1.0/token",
                domain_prefix,
            ))
            .form(&[
                ("refresh_token", refresh_token.as_str()),
                ("client_id", self.config.client_id.as_str()),
                ("client_secret", self.config.client_secret.as_str()),
                ("grant_type", "refresh_token"),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(LsError::OAuth(format!(
                "Token refresh failed with HTTP {}",
                response.status()
            )));
        }

        let token_response: TokenResponse = response.json().await?;

        // Check the new token is valid.
        token_response.validate()?;

        let new_tokens: Tokens = Tokens::try_from(token_response)?;

        {
            let mut tokens = self.tokens.write().await;
            *tokens = new_tokens;
        }

        self.save().await?;

        Ok(())
    }

    pub async fn load<P: AsRef<Path> + Into<String>>(
        token_path: P,
        config: &Config,
        http: reqwest::Client,
    ) -> LsResult<Self> {
        let contents = tokio::fs::read_to_string(&token_path).await?;

        let tokens: Tokens = serde_json::from_str(&contents)?;

        let base_url = tokens.base_url()?;

        Ok(Self {
            config: Arc::new(config.clone()),
            tokens: Arc::new(RwLock::new(tokens)),
            base_url,
            token_path: token_path.into(),
            http,
        })
    }

    pub async fn save_to_path<P: AsRef<Path>>(&self, token_path: P) -> LsResult<()> {
        let tokens = self.tokens.read().await;

        let json = serde_json::to_string_pretty(&*tokens)?;

        tokio::fs::write(token_path, json).await?;

        Ok(())
    }

    pub async fn save(&self) -> LsResult<()> {
        self.save_to_path(self.token_path.clone()).await
    }
}
