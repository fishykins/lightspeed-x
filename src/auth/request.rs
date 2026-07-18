use url::Url;

use crate::auth::{AUTHORIZE_URL, Config, Scope};

/// A helper struct to authenticate the link to a retailer.
#[derive(Debug, Clone)]
pub struct AuthorizationRequest {
    pub state: String,
    pub scopes: Vec<Scope>,
}

impl AuthorizationRequest {
    pub fn new(state: impl Into<String>, scopes: Vec<Scope>) -> Self {
        Self {
            state: state.into(),
            scopes,
        }
    }

    pub fn url(&self, config: &Config) -> Url {
        let mut url =
            Url::parse(AUTHORIZE_URL).expect("Lightspeed authorization URL should always be valid");

        let scope = Scope::join(&self.scopes);

        url.query_pairs_mut()
            .append_pair("response_type", "code")
            .append_pair("client_id", &config.client_id)
            .append_pair("redirect_uri", config.redirect_uri.as_str())
            .append_pair("state", &self.state)
            .append_pair("scope", &scope);

        url
    }
}
