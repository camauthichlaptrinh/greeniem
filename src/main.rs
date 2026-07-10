mod api;
mod app;
mod components;
mod format;
mod pages;
mod route;
mod state;
mod types;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
