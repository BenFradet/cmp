use web_sys::{EventTarget, HtmlInputElement};
use web_sys::wasm_bindgen::JsCast;
use yew::suspense::SuspensionResult;
use yew::prelude::*;

use crate::hooks::use_bool_toggle::use_bool_toggle;
use crate::state::State;

#[derive(Properties, PartialEq)]
pub struct ContentProp {
    pub input: AttrValue,
    pub get_input: Callback<(), String>
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

#[function_component]
fn Content(prop: &ContentProp) -> HtmlResult {
    println!("input: {}", prop.input.as_str());
    let strs = use_results(prop.input.as_str())?;

    Ok(html! {
        <div>{"res: "}{strs}</div>
    })
}

#[function_component]
pub fn App() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    let input_value_handle = use_state(|| "".to_string());

    let input_ref = use_node_ref();

    let search_toggle = use_bool_toggle(false);
    let is_searching = *search_toggle;

    let get_input = {
        let input_ref = input_ref.clone();
        Callback::from(move |_: ()| input_ref.cast::<HtmlInputElement>().unwrap().value())
    };

    let oninput = {
        let state = input_value_handle.clone();
        Callback::from(move |e: InputEvent| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");
            state.set(target.unchecked_into::<HtmlInputElement>().value());
        })
    };

    let on_click = {
        println!("clicking");
        println!("is searching {is_searching}");
        let state = input_value_handle.clone();
        Callback::from(move |_: MouseEvent| {
            state.set("".to_string());
        })
        //move |_: MouseEvent| {
        //    search_toggle.clone().toggle();
        //    println!("is searching {is_searching}");
        //}
    };

    html! {
        <section class="section">
            <div class="container">
                <div class="field has-addons">
                    <div class="control has-icons-left has-icons-right is-expanded">
                        <input ref={input_ref} type="text" class="input is-info is-large" placeholder="Selle italia slr boost endurance" {oninput} value={(*input_value_handle).clone()} />
                        <span class="icon is-left">
                            <i class="fas fa-magnifying-glass"></i>
                        </span>
                    </div>
                    <div class="control">
                        <a class="button is-info is-large"><button class="button is-primary" onclick={on_click}>{"Search"}</button></a>
                    </div>
                </div>
                {
                    html!{
                        <Suspense {fallback}>
                            <Content input={(*input_value_handle).clone()} get_input={get_input} />
                        </Suspense>
                    }
                }
            </div>
        </section>
    }
}