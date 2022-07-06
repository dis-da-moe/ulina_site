use components::App;

mod backend;
mod components;
mod event_bus;
mod util;
mod viewbox;

fn main() {
    yew::start_app::<App>();
}
