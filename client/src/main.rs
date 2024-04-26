use app::App;
use yew::Renderer;

pub mod app;
pub mod components;

fn main() {
    Renderer::<App>::new().render();
}