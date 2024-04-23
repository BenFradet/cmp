use web_sys::HtmlInputElement;
use yew::events::KeyboardEvent;
use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct InputProps {
    pub on_search: Callback<String>,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let onkeypress = {
        let ontrigger = props.on_search.clone();

        move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                println!("enter");
                let input: HtmlInputElement = e.target_unchecked_into();
                let value = input.value();
                println!("value: {value}");

                input.set_value("");
                ontrigger.emit(value);
            }
        }
    };

    let onclick = {
        let ontrigger = props.on_search.clone();

        move |e: MouseEvent| {
            println!("btn click");
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            println!("value: {value}");

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
                <a class="button is-info is-large"><button class="button is-primary" {onclick}>{"Search"}</button></a>
            </div>
        </div>
    }
}