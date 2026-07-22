use crate::{
    LsResult,
    auth::{AuthorizationRequest, Config},
};

use std::{fs, path::Path, sync::Arc};

use axum::{
    Router,
    extract::{Query, State},
    response::Html,
    routing::get,
};
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, oneshot};

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorizationCallback {
    pub code: String,
    pub domain_prefix: String,
    pub state: String,
    pub scope: String,
}

struct CallbackState {
    sender: Mutex<Option<oneshot::Sender<AuthorizationCallback>>>,
}

/// A helper struct to handle app authentification with the retail platform. Only needs to be run
/// once in a while, and is not the actual api.
pub struct LocalCallbackServer;

impl LocalCallbackServer {
    pub async fn authenticate(
        config: &Config,
        request: &AuthorizationRequest,
    ) -> LsResult<AuthorizationCallback> {
        // Channel used to hand the callback back to us.
        let (tx, rx) = oneshot::channel();

        let state = Arc::new(CallbackState {
            sender: Mutex::new(Some(tx)),
        });

        let app = Router::new()
            .route("/callback", get(LocalCallbackServer::callback))
            .with_state(state);

        let host = config
            .redirect_uri
            .host_str()
            .expect("redirect URI must have a host");

        let port = config
            .redirect_uri
            .port_or_known_default()
            .expect("redirect URI must have a port");

        let listener = tokio::net::TcpListener::bind((host, port)).await?;

        // Run the server in the background.
        let server = tokio::spawn(async move { axum::serve(listener, app).await });

        // Open the browser.
        webbrowser::open(request.url(config).as_str()).expect("browser failed to open");

        // Wait for the callback.
        let callback = rx.await.expect("callback failed");

        // Stop the server.
        server.abort();

        Ok(callback)
    }

    async fn callback(
        State(state): State<Arc<CallbackState>>,
        Query(callback): Query<AuthorizationCallback>,
    ) -> Html<&'static str> {
        if let Some(sender) = state.sender.lock().await.take() {
            let _ = sender.send(callback);
        }

        Html(
            "<h2>Authentication successful.</h2>\
         <p>You may now close this window.</p>",
        )
    }
}

impl AuthorizationCallback {
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
        let path = path.as_ref();
        let json = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&json)?)
    }
}
