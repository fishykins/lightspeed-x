use lightspeed_x::{auth::Config, http::LightspeedClient};

#[tokio::main]
async fn main() {
    let config = Config::from_env();
    let client = LightspeedClient::new(config, "tokens/valleysawmills.json")
        .await
        .expect("no client :(");

    /*
    let products = client
        .products()
        .get_all()
        .await
        .expect("failed to get product");

    for product in products.data {
        println!("{:?} -{:?}", product.name, product.sku);
        tokio::fs::write(
            format!("cache/products/{}.json", product.id),
            serde_json::to_string_pretty(&product).expect("failed serdify json to string pretty"),
        )
        .await
        .expect("I dont know but it went wrong");
    }
    */

    let product = client
        .products()
        .get("2cb8d5cd-01f8-4096-9cf8-3d35105a34bb")
        .await;
    println!("Product: {:?}", product);
}
