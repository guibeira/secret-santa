mod api;
mod app;
mod components;

use app::Wrap;

fn main() {
    yew::Renderer::<Wrap>::new().render();
}
