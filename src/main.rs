use crawl::Crawl;
use provider::alltricks::Alltricks;

mod crawl;
mod er;
mod price;
mod provider;
mod search;
mod urls;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let alltricks = Alltricks::default();
    let price = alltricks.crawl("Selle italia slr boost endurance").await?;
    println!("{price}");
    Ok(())
}
