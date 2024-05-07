use yew::prelude::*;

#[function_component(ProgressBar)]
pub fn progress_bar() -> Html {
    html! {
        <div class="container">
            <section class="hero">
                <div class="hero-body">
                    <p class="title">{"Loading..."}</p>
                    <progress class="progress is-large is-info" max="100">{"60%"}</progress>
                </div>
            </section>
        </div>
    }
}