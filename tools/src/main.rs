use yew::prelude::*;
use yew_router::prelude::*;

mod backend;
mod error;
mod event_bus;
mod loader;
mod loading;
mod map;
mod nation;
mod nations;
mod time;
mod util;
mod viewbox;
mod show_nation;
mod flag;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/tools")]
    Home,
    #[at("/tools/map")]
    Map,
    #[at("/tools/time")]
    Time,
    #[at("/tools/nations")]
    Nations,
    #[at("/tools/nation/:id")]
    Nation { id: i64 },
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <App/>
        },
        Route::Map => html! {
            <map::App/>
        },
        Route::Time => html! {
            <time::App/>
        },
        Route::Nations => html! {
            <nations::App/>
        },
        Route::Nation { id } => html! {
            <nation::App id={*id}/>
        },
    }
}

pub struct App;

impl Component for App {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
            <div>
                <Link<Route> to={Route::Map}>{"map"}</Link<Route>>
            </div>
            <div>
                <Link<Route> to={Route::Time}>{"time"}</Link<Route>>
            </div>
            <div>
                <Link<Route> to={Route::Nations}>{"nations"}</Link<Route>>
            </div>
            </>
        }
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
