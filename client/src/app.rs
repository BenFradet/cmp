use domain::response::Response;
use gloo::net::http::Request;
use web_sys::console;
use yew::prelude::*;
use yew::suspense::use_future;

use crate::components::input::Input;

#[derive(Properties, PartialEq)]
pub struct ContentProp {
    pub input: String,
}

const INITIAL_SEARCH_TERM: &'static str = "Selle italia slr boost endurance";

#[function_component(ListingClientInner)]
fn listing_client_inner(prop: &ContentProp) -> HtmlResult {
    let q = urlencoding::encode(&prop.input);
    let url = format!("http://localhost:3030/api/v1/search?q={q}");
    let res = use_future(|| async move {
        Request::get(&url)
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

#[function_component(ListingClient)]
fn listing_client(prop: &ContentProp) -> Html {
    let fallback = html!({ "loading..." });
    // find a way to pass down props
    html!(
        <Suspense {fallback}>
            <ListingClientInner input={prop.input.clone()} />
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
                <ListingClient input={(*input).clone()} />
                <Button />
            </div>
        </section>
    }
}