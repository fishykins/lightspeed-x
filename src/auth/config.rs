use std::{env, path::PathBuf, str::FromStr};
use url::Url;

use crate::auth::Tokens;

#[derive(Debug, Clone)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: Url,
    pub domain_prefix: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let client_id = env::var("LIGHTSPEED_CLIENT_ID").expect("LIGHTSPEED_CLIENT_ID not set");

        let client_secret =
            env::var("LIGHTSPEED_CLIENT_SECRET").expect("LIGHTSPEED_CLIENT_SECRET not set");

        let redirect_uri = Url::from_str(
            &env::var("LIGHTSPEED_REDIRECT_URI").expect("LIGHTSPEED_REDIRECT_URI not set"),
        )
        .expect("Failed to parse redirect url");

        let domain_prefix =
            env::var("LIGHTSPEED_DOMAIN_PREFIX").expect("LIGHTSPEED_DOMAIN_PREFIX not set");

        println!("{:?}", redirect_uri);

        Self {
            client_id,
            client_secret,
            redirect_uri,
            domain_prefix,
        }
    }

    pub fn domain_path(&self) -> PathBuf {
        Tokens::path_from_domain(&self.domain_prefix).into()
    }
}
