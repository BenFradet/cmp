use web_sys::console;
use yew::prelude::*;

use crate::components::{entries::Entries, input::Input, progress_bar::ProgressBar};

#[function_component(App)]
pub fn app() -> Html {
    let fallback = html! { <ProgressBar /> };
    let input = use_state_eq(|| Option::<String>::None);

    let on_search = {
        let input = input.clone();
        Callback::from(move |s| {
            console::log_1(&format!("setting state: {:?}", s).into());
            input.set(s);
        })
    };

    html! {
        <div class="columns">
            <div class="column">{"Ad here"}</div>
            <div class="column is-four-fifths">
                <section class="section">
                    <div class="container">
                        <Input {on_search} />
                        <Suspense {fallback}>
                            <Entries input={(*input).clone()} />
                        </Suspense>
                    </div>
                </section>
            </div>
            <div class="column">{"Ad here"}</div>
        </div>
    }
}