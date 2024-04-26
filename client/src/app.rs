use domain::response::Response;
use gloo::net::http::Request;
use web_sys::console;
use yew::prelude::*;
use yew::suspense::use_future;

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
            //.mode(web_sys::RequestMode::Cors)
            .send()
            .await?
            .json::<Response>()
            .await
    })?;
    let result_html = match *res {
        Ok(ref res) => html! { format!("{:?}", res) },
        Err(ref failure) => failure.to_string().into(),
    };
    Ok(result_html)
}

#[function_component(Listing)]
fn listing(prop: &ContentProp) -> Html {
    let fallback = html!({ "loading..." });
    // find a way to pass down props
    html!(
        <Suspense {fallback}>
            <ListingInner input={prop.input.clone()} />
        </Suspense>
    )
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
                <Button />
            </div>
        </section>
    }
}