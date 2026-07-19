use lightspeed_x::{auth::Config, http::LightspeedClient};

#[tokio::main]
async fn main() {
    let config = Config::from_env();
    let client = LightspeedClient::new(config, "tokens/valleysawmills.json")
        .await
        .expect("no client :(");

    let product = client
        .products()
        .get_all()
        .await
        .expect("failed to get product");

    println!("Product found: {}", product);
}
