use domain::response::Response;
use gloo::net::http::Request;
use web_sys::console;
use yew::prelude::*;
use yew::suspense::use_future;

use crate::components::entry::Entry;
use crate::components::input::Input;

#[derive(Properties, PartialEq)]
pub struct ContentProp {
    pub input: Option<AttrValue>,
}

const INITIAL_SEARCH_TERM: &'static str = "Selle italia slr boost endurance";

#[function_component(ListingInner)]
fn listing_inner(prop: &ContentProp) -> HtmlResult {
    let input = prop.input.clone();
    let q = input.unwrap_or(AttrValue::Static(INITIAL_SEARCH_TERM)).clone();
    let encoded_q = urlencoding::encode(&q);
    let url = format!("/api/v1/search?q={encoded_q}");
    let res = use_future(|| async move {
        console::log_1(&format!("sending request to url: {url}").into());
        // investigate cache
        Request::get(&url)
            .send()
            .await?
            .json::<Response>()
            .await
    })?;
    let result_html = match *res {
        Ok(ref res) =>
            if res.items.is_empty() {
                html! { "not found" }
            } else {
                // find a way without clone
                let mut items = res.items.clone();
                items.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
                items.iter().map(|item| html! {
                    <Entry item={item.clone()} />
                }).collect::<Html>()
            },
        Err(ref failure) => {
            console::log_1(&format!("failure to receive response: {failure}").into());
            failure.to_string().into()
        },
    };
    Ok(result_html)
}

#[function_component(Listing)]
fn listing(prop: &ContentProp) -> Html {
    // replace by bulma progress bar
    let fallback = html! {
        <div class="container">
            <section class="hero">
                <div class="hero-body">
                    <p class="title">{"Loading..."}</p>
                    <progress class="progress is-large is-info" max="100">{"60%"}</progress>
                </div>
            </section>
        </div>
    };
    // find a way to pass down props
    html!(
        <Suspense {fallback}>
            <div class="columns is-multiline is-mobile">
                <ListingInner input={prop.input.clone()} />
            </div>
        </Suspense>
    )
}

#[function_component(App)]
pub fn app() -> Html {
    let input = use_state_eq(|| Option::<String>::None);

    let on_search = {
        let input = input.clone();
        Callback::from(move |s| {
            console::log_1(&format!("setting state: {:?}", s).into());
            input.set(s);
        })
    };

    html! {
        <section class="section">
            <div class="container">
                <Input {on_search} />
                <Listing input={(*input).clone()} />
            </div>
        </section>
    }
}