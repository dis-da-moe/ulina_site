use route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

mod backend;
mod event_bus;
mod home;
mod map;
mod route;
mod time;
mod util;
mod viewbox;

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <home::App/>
        },
        Route::Map => html! {
            <map::App/>
        },
        Route::Time => html! {
            <time::App/>
        },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<Main>();
}
