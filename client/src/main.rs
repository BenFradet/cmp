use app::App;
use yew::Renderer;

pub mod app;
pub mod components;
pub mod services;

fn main() {
    Renderer::<App>::new().render();
}