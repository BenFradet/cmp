use web_sys::console;
use yew::prelude::*;

use crate::components::{entries::Entries, input::Input, progress_bar::ProgressBar};

#[derive(Properties, PartialEq)]
pub struct ListingProps {
    pub input: Option<AttrValue>,
}

const INITIAL_SEARCH_TERM: &'static str = "Selle italia slr boost endurance";

#[function_component(Listing)]
fn listing(props: &ListingProps) -> Html {
    let fallback = html! { <ProgressBar /> };
    // find a way to pass down props
    html!(
        <Suspense {fallback}>
            <section class="hero">
                <div class="hero-body">
                    <p class="title">{format!("Results for \"{}\":", props.input.clone().unwrap_or(AttrValue::Static(INITIAL_SEARCH_TERM)))}</p>
                </div>
            </section>
            <div class="columns is-multiline is-mobile">
                <Entries input={props.input.clone()} />
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