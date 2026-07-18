use lightspeed_x::{auth::Config, http::LightspeedClient};

#[tokio::main]
async fn main() {
    let config = Config::from_env();
    let client = LightspeedClient::new(config, "tokens/valleysawmills.json")
        .await
        .expect("no client :(");

    client
        .blast_api_with_nonsense()
        .await
        .expect("failed to api, bad luck");
}
