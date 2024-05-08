use web_sys::console;
use yew::prelude::*;
use yew::suspense::use_future_with;

use crate::services::search::search;
use crate::components::entry::Entry;
use crate::components::hero::Hero;

#[derive(PartialEq, Properties, Clone)]
pub struct EntriesProps {
    pub input: Option<AttrValue>,
}

const INITIAL_SEARCH_TERM: &'static str = "Selle italia slr boost endurance";

#[function_component(Entries)]
pub fn entries(props: &EntriesProps) -> HtmlResult {
    let input = props.input.clone();
    let results = use_future_with(input, |i| async move {
        search(i.as_deref()).await
    })?;

    let result_html = match *results {
        Ok(ref items) =>
            if items.is_empty() {
                html! { "not found" }
            } else {
                // todo: find a way without clone
                let mut items = items.clone();
                items.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

                let list = items.iter().map(|item| html! {
                    <Entry item={item.clone()} />
                }).collect::<Html>();

                let result_text = props.input.clone()
                    .unwrap_or(AttrValue::Static(INITIAL_SEARCH_TERM));
                let txt = format!("Results for \"{}\":", result_text);

                html! {
                    <div>
                        <Hero text={txt} />
                        <div class="columns is-multiline is-mobile">
                            {list}
                        </div>
                    </div>
                }
            },
        Err(ref failure) => {
            console::log_1(&format!("failure to receive response: {failure}").into());
            failure.to_string().into()
        },
    };
    Ok(result_html)
}

