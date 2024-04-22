use web_sys::HtmlInputElement;
use yew::events::KeyboardEvent;
use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct InputProps {
    pub ontrigger: Callback<String>,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let onkeypress = {
        let ontrigger = props.ontrigger.clone();

        move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: HtmlInputElement = e.target_unchecked_into();
                let value = input.value();

                input.set_value("");
                ontrigger.emit(value);
            }
        }
    };

    let onclick = {
        let ontrigger = props.ontrigger.clone();

        move |e: MouseEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();

            input.set_value("");
            ontrigger.emit(value);
        }
    };

    html! {
        <div class="field has-addons">
            <div class="control has-icons-left has-icons-right is-expanded">
                <input type="text" class="input is-info is-large" placeholder="Selle italia slr boost endurance" {onkeypress} />
                <span class="icon is-left">
                    <i class="fas fa-magnifying-glass"></i>
                </span>
            </div>
            <div class="control">
                <a class="button is-info is-large"><button class="button is-primary" onclick={onclick}>{"Search"}</button></a>
            </div>
        </div>
    }
}