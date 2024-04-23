use yew::suspense::SuspensionResult;
use yew::prelude::*;

use crate::components::input::Input;
use crate::domain::state::State;

#[derive(Properties, PartialEq)]
pub struct ContentProp {
    pub input: String,
}

#[hook]
fn use_results(search: &str) -> SuspensionResult<Vec<String>> {
    let state = use_state(|| State::new(search));
    let result = match *state.value.borrow() {
        Some(ref strs) => Ok(strs.clone()),
        None => Err(state.susp.clone()),
    };
    result
}

#[function_component(Content)]
fn content(prop: &ContentProp) -> HtmlResult {
    let strs = use_results(prop.input.as_str())?;

    Ok(html! {
        <div>{"res: "}{strs}</div>
    })
}

#[function_component(App)]
pub fn app() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    let input = use_state(|| "".to_string());

    let on_search = {
        let input = input.clone();
        move |s| {
            println!("setting state {s}");
            input.set(s);
        }
    };

    html! {
        <section class="section">
            <div class="container">
                <Input {on_search} />
                <h1>{(*input).clone()}</h1>
                //<Suspense {fallback}>
                //    <Content input={(*input).clone()} />
                //</Suspense>
            </div>
        </section>
    }
}