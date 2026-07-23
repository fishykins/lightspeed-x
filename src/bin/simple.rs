use lightspeed_x::{LsResult, auth::Config, http::LightspeedClient, models::sales::SaleState};

#[tokio::main]
async fn main() -> LsResult<()> {
    let config = Config::from_env();
    let client = LightspeedClient::from_config(config)
        .await
        .expect("no client :(");

    /*
    let products = client
        .products()
        .get_all()
        .await
        .expect("failed to get product"); w

    for product in products.data {
        println!("{:?} -{:?}", product.name, product.sku);
        tokio::fs::write(
            format!("cache/products/{}.json", product.id),
            serde_json::to_string_pretty(&product).expect("failed serdify json to string pretty"),
        )
        .await
        .expect("I dont know but it went wrong");
    }

    let product_result = client
        .products()
        .get("2cb8d5cd-01f8-4096-9cf8-3d35105a34bb")
        .await?;

    let product = product_result.data;

    println!("{}: {:?}", product.sku, product.handle);

    */

    let sales_result = client.sales().get_all().await?;

    //let json = tokio::fs::read_to_string("cache/api/sales.json").await?;
    //let sales_result: ListResponse<Sale> = serde_json::from_str(&json)?;

    let mut total = 0.0;
    let mut closed = 0.0;
    let mut open = 0.0;
    let mut completed = 0.0;

    for sale in sales_result.data.iter() {
        if let Some(totals) = &sale.totals {
            let x = totals.price_incl_tax;
            total += x;
            match sale.state {
                SaleState::Open => open += x,
                SaleState::Closed => closed += x,
                SaleState::Completed => completed += x,
                _ => total += -x,
            }
        }
    }

    println!("Total sales: £{:.2}", total);
    println!("Closed sales: £{:.2}", closed);
    println!("Open sales: £{:.2}", open);
    println!("Completed sales: £{:.2}", completed);

    sales_result.save_to_file("cache/sales.json").await?;

    Ok(())
}
