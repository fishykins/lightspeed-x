use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;

use url::Url;

use crate::{
    LsResult,
    auth::{Config, Tokens},
};

pub struct Authenticator {
    pub(crate) config: Arc<Config>,
    pub(crate) tokens: Arc<RwLock<Tokens>>,
    token_path: String, // TODO: Make PathBuf
    base_url: Url,
}

impl Authenticator {
    pub async fn bearer_token(&self) -> LsResult<String> {
        let tokens = self.tokens.read().await;

        // TODO: Check we haven't expired!

        Ok(tokens.access_token.clone())
    }

    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    async fn refresh(&self) -> LsResult<()> {
        todo!()
    }

    pub async fn load<P: AsRef<Path> + Into<String>>(
        token_path: P,
        config: &Config,
    ) -> LsResult<Self> {
        let contents = tokio::fs::read_to_string(&token_path).await?;

        let tokens: Tokens = serde_json::from_str(&contents)?;

        let base_url = tokens.base_url()?;

        Ok(Self {
            config: Arc::new(config.clone()),
            tokens: Arc::new(RwLock::new(tokens)),
            base_url,
            token_path: token_path.into(),
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
