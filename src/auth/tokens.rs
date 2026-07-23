use crate::{LsError, LsResult, auth::Scope};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenRequest<'a> {
    pub code: &'a str,
    pub client_id: &'a str,
    pub client_secret: &'a str,
    pub grant_type: &'static str,
    pub redirect_uri: &'a str,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    access_token: String,
    token_type: String,
    expires: i64,
    expires_in: u64,
    refresh_token: String,
    domain_prefix: String,
    scope: String,
}

impl TryFrom<TokenResponse> for Tokens {
    type Error = LsError;

    fn try_from(response: TokenResponse) -> Result<Self, Self::Error> {
        let expires_at = DateTime::<Utc>::from_timestamp(response.expires, 0)
            .ok_or_else(|| LsError::OAuth("Invalid expiry timestamp".into()))?;

        Ok(Self {
            access_token: response.access_token,
            refresh_token: response.refresh_token,
            expires_at,
            domain_prefix: response.domain_prefix,
            scope: Scope::parse(&response.scope),
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tokens {
    pub access_token: String,
    pub expires_at: DateTime<Utc>,
    pub refresh_token: String,
    pub domain_prefix: String,
    pub scope: Vec<Scope>,
}

impl Tokens {
    pub fn save<P: AsRef<Path>>(&self, path: P) -> LsResult<()> {
        let path = path.as_ref();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(self)?;

        fs::write(path, json)?;

        Ok(())
    }

    pub fn default_path(&self) -> String {
        Self::path_from_domain(&self.domain_prefix)
    }

    pub fn path_from_domain(domain_prefix: impl Into<String>) -> String {
        format!("tokens/{}.json", domain_prefix.into())
    }

    pub fn load<P: AsRef<Path>>(path: P) -> LsResult<Self> {
        let json = fs::read_to_string(path)?;

        Ok(serde_json::from_str(&json)?)
    }

    /// Returns the base Url without the api version appended.
    /// Oauth and token calls often use 2.0 or 1.0, wheras main api calls
    /// use more detailed verison control.
    pub fn base_url(&self) -> LsResult<Url> {
        Ok(Url::parse(&format!(
            "https://{}.retail.lightspeed.app/api/",
            self.domain_prefix
        ))?)
    }

    pub fn needs_refresh(&self) -> bool {
        Utc::now() >= self.expires_at - chrono::Duration::seconds(60)
    }
}

impl TokenResponse {
    pub fn validate(&self) -> LsResult<()> {
        if self.access_token.is_empty() {
            return Err(LsError::OAuth(
                "OAuth server returned an empty access token".into(),
            ));
        }

        if self.refresh_token.is_empty() {
            return Err(LsError::OAuth(
                "OAuth server returned an empty refresh token".into(),
            ));
        }

        Ok(())
    }

    pub fn access_token(&self) -> &str {
        &self.access_token
    }
}
