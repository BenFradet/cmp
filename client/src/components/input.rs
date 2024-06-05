use web_sys::{console, HtmlInputElement};
use yew::events::KeyboardEvent;
use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct InputProps {
    pub on_search: Callback<Option<String>>,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let onkeypress = {
        let ontrigger = props.on_search.clone();

        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                console::log_1(&"enter".into());
                let input: HtmlInputElement = e.target_unchecked_into();
                let value = input.value();
                console::log_1(&format!("value: {}", value).into());

                input.set_value("");
                ontrigger.emit(Some(value));
            }
        })
    };

    let input_ref = use_node_ref();

    let onclick = {
        let ontrigger = props.on_search.clone();
        let ir = input_ref.clone();

        Callback::from(move |_: MouseEvent| {
            console::log_1(&"btn click".into());
            let input: HtmlInputElement = ir.cast::<HtmlInputElement>().unwrap();
            let value = input.value();
            console::log_1(&format!("value: {}", value).into());

            input.set_value("");
            ontrigger.emit(Some(value));
        })
    };

    html! {
        <div class="field has-addons">
            <div class="control has-icons-left has-icons-right is-expanded">
                <input ref={input_ref} type="text" class="input is-info is-large" placeholder="slr boost endurance" {onkeypress} />
                <span class="icon is-left">
                    <i class="fas fa-magnifying-glass"></i>
                </span>
            </div>
            <div class="control">
                <a class="button is-info is-large is-primary"><button {onclick}>{"Search"}</button></a>
            </div>
        </div>
    }
}