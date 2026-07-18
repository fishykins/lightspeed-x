use crate::{LsError, LsResult, auth::Scope};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

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
        Ok(Self {
            access_token: response.access_token,
            refresh_token: response.refresh_token,
            expires: response.expires,
            expires_in: response.expires_in,
            domain_prefix: response.domain_prefix,
            scope: Scope::parse(&response.scope),
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tokens {
    pub access_token: String,
    pub expires: i64,
    pub expires_in: u64,
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

    pub fn load<P: AsRef<Path>>(path: P) -> LsResult<Self> {
        let json = fs::read_to_string(path)?;

        Ok(serde_json::from_str(&json)?)
    }
}
