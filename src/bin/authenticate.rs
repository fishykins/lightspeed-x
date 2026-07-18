use lightspeed_x::auth::{
    AuthorizationRequest, Config, LocalCallbackServer, OAuthClient, Scope, Tokens,
};

#[tokio::main]
async fn main() {
    let config = Config::from_env();

    let auth_request = AuthorizationRequest::new("testymctest", vec![Scope::SalesRead]);

    let url = auth_request.url(&config);
    println!("Url: {url}");

    let authorization_callback = LocalCallbackServer::authenticate(&config, &auth_request)
        .await
        .expect("Failed to start server");

    println!("callback: {:?}", authorization_callback);

    let tokens: Tokens = OAuthClient::new(&config)
        .exchange_code(&authorization_callback)
        .await
        .expect("Failed Oauth");

    println!("OAuth Tokens: {:?}", tokens);

    tokens
        .save(format!("tokens/{}.json", tokens.domain_prefix))
        .expect("Failed to save token to file, this really was a fall at the last hurdle");
}
