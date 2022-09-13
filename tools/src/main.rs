use crate::components::Login;
use async_trait::async_trait;
use common::UserData;
use loader::{LoadHandler, LoadProps, Loader};
use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::button;
mod backend;
mod components;
mod display;
mod loader;
mod pages;
mod util;
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
    #[at("/tools/changes")]
    Changes,
    #[at("/tools/create-map")]
    CreateMap,
    #[at("/tools/create-nation")]
    CreateNation,
    #[not_found]
    #[at("/404")]
    NotFound
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <App/>
        },
        Route::Map => html! {
            <pages::map::App/>
        },
        Route::Time => html! {
            <pages::time::App/>
        },
        Route::Nations => html! {
            <pages::nations::App/>
        },
        Route::Nation { id } => html! {
            <pages::nation::App id={*id}/>
        },
        Route::Changes => html! {
            <pages::changes::App/>
        },
        Route::CreateMap => html! {
            <pages::create_map::App/>
        },
        Route::CreateNation => html!{
            <pages::create_nation::App/>
        },
        Route::NotFound => html!{
            {"not found"}
        }
    }
}

pub struct Home;
type HomeProps = LoadProps<UserData>;
type App = Loader<UserData, Home>;

#[async_trait(?Send)]
impl LoadHandler<UserData> for Loader<UserData, Home> {
    async fn load() -> Result<UserData, String> {
        backend::user_data().await
    }
}

impl Component for Home {
    type Message = ();

    type Properties = HomeProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let user = &ctx.props().loaded;
        html! {
            <>
            {navbar!()}
            <Login user={user.clone()}/>
            {button(Route::Map, "map")}
            {button(Route::Time, "time")}
            {button(Route::Nations, "nations")}
            if user.isAdmin{
                {button(Route::Changes, "change")}
                {button(Route::CreateMap, "create map")}
                {button(Route::CreateNation, "create nation")}
            }
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
