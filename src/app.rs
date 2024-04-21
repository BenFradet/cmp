use web_sys::{EventTarget, HtmlInputElement};
use web_sys::wasm_bindgen::JsCast;
use yew::suspense::SuspensionResult;
use yew::prelude::*;

use crate::state::State;

#[derive(Properties, PartialEq)]
pub struct ContentProp {
    pub input: String,
    pub get_input: Callback<(), String>
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
fn Content(prop: &ContentProp) -> HtmlResult {
    let strs = use_results()?;

    Ok(html! {
        <div>{"res: "}{strs}</div>
    })
}

#[function_component]
pub fn App() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    let input_value_handle = use_state(|| "".to_string());

    let input_ref = use_node_ref();

    let get_input = {
        let input_ref = input_ref.clone();
        Callback::from(move |_: ()| input_ref.cast::<HtmlInputElement>().unwrap().value().to_uppercase())
    };

    let oninput = {
        let state = input_value_handle.clone();
        Callback::from(move |e: InputEvent| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");
            state.set(target.unchecked_into::<HtmlInputElement>().value().to_uppercase());
        })
    };

    let clear_input = {
        let state = input_value_handle.clone();
        Callback::from(move |_: MouseEvent| {
            state.set("".to_string());
        })
    };

    html! {
        <section class="section">
            <div class="container">
                <div class="field has-addons">
                    <div class="control has-icons-left has-icons-right is-expanded">
                        <input ref={input_ref} type="text" class="input is-info is-large" placeholder="Search" {oninput} value={(*input_value_handle).clone()} />
                        <span class="icon is-left">
                            <i class="fas fa-magnifying-glass"></i>
                        </span>
                    </div>
                    <div class="control">
                        <a class="button is-info is-large"><button class="delete is-large" onclick={clear_input}></button></a>
                    </div>
                </div>
                <Suspense {fallback}>
                    <Content input={(*input_value_handle).clone()} get_input={get_input} />
                </Suspense>
            </div>
        </section>
    }
}