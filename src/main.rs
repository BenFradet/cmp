use std::convert::Infallible;

use tokio::task::LocalSet;
use warp::Filter;

use crate::app::App;

mod app;
mod hooks;
mod provider;
mod state;

async fn render() -> Result<impl warp::Reply, Infallible> {
    let content = tokio::task::spawn_blocking(move || {
        use tokio::runtime::Builder;
        let set = LocalSet::new();

        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        set.block_on(&rt, async {
            let renderer = yew::ServerRenderer::<App>::new();

            renderer.render().await
        })
    })
    .await
    .expect("the thread has failed");

    Ok(
        warp::reply::html(
            format!(
                r#"<!DOCTYPE html>
                <html>
                <head>
                    <meta charset="utf-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1">
                    <title>Comparo-Cyclo</title>
                    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma@0.9.4/css/bulma.min.css">
                    <script src="https://kit.fontawesome.com/ee3fa7d08f.js" crossorigin="anonymous"></script>
                </head>
                <body>
                    <h1>comparo-cyclo</h1>
                    {}
                </body>
                </html>"#,
                content
            )
        )
    )
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> () {
    let routes = warp::path::end().and_then(|| render());
    println!("running at localhost:3030");
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}