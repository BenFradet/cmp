use futures::{stream, StreamExt};
use provider::Provider;
use reqwest::Client;

mod provider;

const CONCURRENT_REQUESTS: usize = 2;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::builder().build()?;

    let search = "Selle italia slr boost endurance";

    let providers = vec![Provider::BIKE_DISCOUNT, Provider::ALLTRICKS, Provider::STARBIKE];

    let results = stream::iter(providers)
        .map(|p| {
            let client = &client;
            async move { p.crawl(client, &search).await }
        })
        .buffer_unordered(CONCURRENT_REQUESTS);

    results.for_each(|r| async {
        match r {
            Ok(r) => println!("{r}"),
            Err(e) => println!("Err: {e}"),
        }
    }).await;
    Ok(())
}
