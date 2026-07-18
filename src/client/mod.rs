use reqwest::Client as HttpClient;

use crate::auth::{Config, Tokens};

pub struct LightspeedClient {
    pub config: Config,
    pub tokens: Tokens,
    pub http: HttpClient,
}

impl LightspeedClient {
    pub async fn get(&self, endpoint: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!(
            "https://{}.retail.lightspeed.app/api/2.0{}",
            self.tokens.domain_prefix, endpoint,
        );

        self.http
            .get(url)
            .bearer_auth(&self.tokens.access_token)
            .send()
            .await
    }
}
