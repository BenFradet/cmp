use provider::Provider;
use reqwest::Client;

mod er;
mod provider;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let search = "Selle italia slr boost endurance";


    let providers = vec![Provider::BIKE_DISCOUNT, Provider::ALLTRICKS, Provider::STARBIKE];
    let mut tasks = Vec::with_capacity(providers.len());
    for p in providers {
        tasks.push(tokio::spawn(p.crawl(&client, &search)));
    }

    let mut outputs = Vec::with_capacity(tasks.len());
    for task in tasks {
        outputs.push(task.await.unwrap());
    }

    println!("{:?}", outputs);
    Ok(())
    //let crawls = providers.map(|p| p.crawl(&client, search));
    //let results = 

    //let bd = Provider::BIKE_DISCOUNT;
    //let bd_price = bd.crawl(&client, search).await?;
    //println!("bd: {bd_price}");

    //let at = Provider::ALLTRICKS;
    //let at_price = at.crawl(&client, search).await?;
    //println!("at: {at_price}");

    //let sb = Provider::STARBIKE;
    //let sb_price = sb.crawl(&client, &search).await?;
    //println!("sb: {sb_price}");

    //Ok(())
}
