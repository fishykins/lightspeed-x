mod callback;
mod config;
mod oauth_client;
mod request;
mod scope;
mod tokens;

pub use callback::{AuthorizationCallback, LocalCallbackServer};
pub use config::Config;
pub use oauth_client::OAuthClient;
pub use request::AuthorizationRequest;
pub use scope::Scope;
pub use tokens::{TokenRequest, Tokens};

pub const AUTHORIZE_URL: &str = "https://secure.retail.lightspeed.app/connect";
