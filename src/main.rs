use std::convert::Infallible;

use tokio::task::LocalSet;
use warp::Filter;
use yew::{prelude::*, suspense::SuspensionResult};

use crate::state::State;

mod provider;
mod state;

#[hook]
fn use_results() -> SuspensionResult<Vec<String>> {
    let state = use_state(State::new);
    let result = match *state.value.borrow() {
        Some(ref strs) => Ok(strs.clone()),
        None => Err(state.susp.clone()),
    };
    result
}

#[function_component]
fn Content() -> HtmlResult {
    let strs = use_results()?;

    Ok(html! {
        <div>{"res: "}{strs}</div>
    })
}

#[function_component]
pub fn App() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <Suspense {fallback}>
            <Content />
        </Suspense>
    }
}

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
                r#"<!DOCTYPE HTML>
                    <html>
                        <head>
                            <title>comparo-cyclo</title>
                        </head>
                        <body>
                            <h1>comparo-cyclo</h1>
                            {}
                        </body>
                    </html>
                "#,
                content
            )
        )
    )
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> () {
    let routes = warp::path::end().and_then(|| render());
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}