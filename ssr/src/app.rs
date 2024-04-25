use futures::{stream, StreamExt};
use reqwest::Client;
use web_sys::{console, HtmlInputElement};
use yew::prelude::*;

use crate::{components::input::Input, domain::provider::Provider};

#[derive(Properties, PartialEq)]
pub struct ContentProp {
    pub input: String,
}

const PARALLEL_REQUESTS: usize = 3;
const INITIAL_SEARCH_TERM: &'static str = "Selle italia slr boost endurance";

#[cfg(feature = "ssr")]
pub async fn fetch(search_term: &str) -> Vec<String> {
    let client = Client::new();

    let providers = vec![Provider::BIKE_DISCOUNT, Provider::ALLTRICKS, Provider::STARBIKE];
    let providers_len = providers.len();

    let results = stream::iter(providers)
        .map(|p| {
            // should be safe to clone since backed by an rc (to check)
            let client = client.clone();
            // corouting moves
            let s = search_term.to_owned();
            tokio::spawn(async move {
                p.crawl(&client, &s).await.unwrap_or("not found".to_owned())
            })
        })
        .buffer_unordered(PARALLEL_REQUESTS)
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

#[function_component(Content)]
fn content(prop: &ContentProp) -> HtmlResult {
    let strs = use_prepared_state!((), async move |_| -> Vec<String> { fetch(INITIAL_SEARCH_TERM).await })?.unwrap();

    Ok(html! {
        { for strs.iter().map(|s|
            html!{<div>{"res: "}{s}</div>}
        )}
    })
}

#[function_component]
fn Button() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };
    let value = *counter;
    html! {
        <button {onclick}>{format!("Clicked {value} times!")}</button>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    let input = use_state_eq(|| "".to_string());

    let on_search = {
        let input = input.clone();
        Callback::from(move |s| {
            console::log_1(&format!("setting state: {}", s).into());
            input.set(s);
        })
    };

    html! {
        <section class="section">
            <div class="container">
                <Input {on_search} />
                <h1>{(*input).clone()}</h1>
                <Suspense {fallback}>
                    <Content input={(*input).clone()} />
                </Suspense>
                <Button />
            </div>
        </section>
    }
}