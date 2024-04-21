use std::{cell::RefCell, convert::Infallible, rc::Rc};

use futures::{stream, StreamExt};
use provider::Provider;
use reqwest::Client;
use tokio::task::{spawn_local, LocalSet};
use warp::Filter;
use yew::{prelude::*, suspense::{Suspension, SuspensionResult}};

mod provider;

const CONCURRENT_REQUESTS: usize = 2;

struct State {
    susp: Suspension,
    value: Rc<RefCell<Option<Vec<String>>>>,
}

impl State {
    fn new() -> Self {
        let (susp, handle) = Suspension::new();
        let value: Rc<RefCell<Option<Vec<String>>>> = Rc::default();

        {
            let value = value.clone();
            // we use tokio spawn local here.
            spawn_local(async move {
                let res = fetch();
                {
                    let mut value = value.borrow_mut();
                    *value = Some(res.await);
                }

                handle.resume();
            });
        }

        Self { susp, value }
    }
}

async fn fetch() -> Vec<String> {
    let client = Client::new();

    let search = "Selle italia slr boost endurance";

    let providers = vec![Provider::BIKE_DISCOUNT, Provider::ALLTRICKS, Provider::STARBIKE];
    let providers_len = providers.len();

    let results = stream::iter(providers)
        .map(|p| {
            // should be safe to clone since backed by an rc (to check)
            let client = client.clone();
            tokio::spawn(async move {
                p.crawl(&client, &search).await.unwrap_or("not found".to_owned())
            })
        })
        .buffer_unordered(CONCURRENT_REQUESTS)
        .take(providers_len)
        .collect::<Vec<_>>()
        .await;

    results
        .into_iter()
        .map(|r| match r {
            Ok(s) => Some(s),
            Err(e) => {
                println!("join error: {e}");
                None
            },
        })
        .flatten()
        .collect()
}

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