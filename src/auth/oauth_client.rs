use crate::{
    LsError, LsResult,
    auth::{AuthorizationCallback, Config, TokenRequest, Tokens, tokens::TokenResponse},
};

pub struct OAuthClient<'a> {
    http: reqwest::Client,
    config: &'a Config,
}

impl<'a> OAuthClient<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            http: reqwest::Client::new(),
            config,
        }
    }

    pub async fn exchange_code(&self, callback: &AuthorizationCallback) -> LsResult<Tokens> {
        let url = format!(
            "https://{}.retail.lightspeed.app/api/1.0/token",
            callback.domain_prefix
        );

        let request = TokenRequest {
            code: &callback.code,
            client_id: &self.config.client_id,
            client_secret: &self.config.client_secret,
            grant_type: "authorization_code",
            redirect_uri: self.config.redirect_uri.as_str(),
        };

        println!("TokenRequest: {:?}", request);

        let response = self.http.post(url).form(&request).send().await?;

        if !response.status().is_success() {
            let body = response.text().await?;

            return Err(LsError::OAuth(body));
        }

        let token_response: TokenResponse = response.json().await?;

        let tokens: Tokens = token_response.try_into()?;

        return Ok(tokens);
    }
}
