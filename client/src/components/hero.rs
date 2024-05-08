use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct HeroProps {
    pub text: AttrValue,
}

#[function_component(Hero)]
pub fn hero(props: &HeroProps) -> Html {
    html! {
        <section class="hero">
            <div class="hero-body">
                <p class="title">{props.text.clone()}</p>
            </div>
        </section>
    }
}