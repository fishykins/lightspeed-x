use lightspeed_x::auth::{
    AuthorizationCallback, AuthorizationRequest, Config, LocalCallbackServer, OAuthClient, Scope,
    Tokens,
};

async fn make_auth_request(config: &Config, path: &str) -> AuthorizationCallback {
    let auth_request = AuthorizationRequest::new(
        "testymctest",
        vec![
            Scope::SalesRead,
            Scope::SalesWrite,
            Scope::ProductsRead,
            Scope::ProductsWrite,
            Scope::CustomersWrite,
            Scope::CustomersWrite,
        ],
    );

    let url = auth_request.url(&config);
    println!("Url: {url}");

    let auth_callback = LocalCallbackServer::authenticate(&config, &auth_request)
        .await
        .expect("failed to request auth code");

    auth_callback
        .save(path)
        .expect("Failed to write auth_callback.json to file");
    auth_callback
}

#[tokio::main]
async fn main() {
    let config = Config::from_env();

    // We create a local cache of the request incase of silly errors later- this api call is
    // expensive and we get rate limited very quickly so if this works, store the data!!!
    let temp_auth_callback_path = format!("tokens/temp_{}.json", config.client_id);

    let auth_callback = AuthorizationCallback::load(&temp_auth_callback_path)
        .unwrap_or(make_auth_request(&config, &temp_auth_callback_path).await);

    let tokens: Tokens = OAuthClient::new(&config)
        .exchange_code(&auth_callback)
        .await
        .expect("Failed token Oauth");

    println!("OAuth Tokens: {:?}", tokens);

    tokens
        .save(format!("tokens/{}.json", tokens.domain_prefix))
        .expect("Failed to save token to file, this really was a fall at the last hurdle");

    std::fs::remove_file(temp_auth_callback_path)
        .expect("failed to clean up but otherwise good job");
}
