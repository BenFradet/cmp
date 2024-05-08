use web_sys::console;
use yew::prelude::*;

use crate::services::search::search;
use crate::components::entry::Entry;
use crate::components::progress_bar::ProgressBar;
use crate::components::hero::Hero;

#[derive(PartialEq, Properties, Clone)]
pub struct EntriesProps {
    pub input: Option<AttrValue>,
}

const INITIAL_SEARCH_TERM: &'static str = "Selle italia slr boost endurance";

#[function_component(Entries)]
pub fn entries(props: &EntriesProps) -> HtmlResult {
    let results = use_state(|| None);

    let input = props.input.clone();
    {
        let input_clone = input.clone();
        let results = results.clone();

        use_effect_with(input, move |_| {
            let input_clone_clone = input_clone.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let res = search(input_clone_clone.as_deref()).await;
                results.set(Some(res));
            });
            || {}
        });
    }

    let result_html = match &(*results) {
        None => html! { <ProgressBar /> },
        Some(Ok(ref items)) =>
            if items.is_empty() {
                html! { "not found" }
            } else {
                // find a way without clone
                let mut items = items.clone();
                items.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
                let list = items.iter().map(|item| html! {
                    <Entry item={item.clone()} />
                }).collect::<Html>();
                let txt = format!("Results for \"{}\":", props.input.clone().unwrap_or(AttrValue::Static(INITIAL_SEARCH_TERM)));
                html! {
                    <div>
                        <Hero text={txt} />
                        <div class="columns is-multiline is-mobile">
                            {list}
                        </div>
                    </div>
                }
            },
        Some(Err(ref failure)) => {
            console::log_1(&format!("failure to receive response: {failure}").into());
            failure.to_string().into()
        },
    };
    Ok(result_html)
}

