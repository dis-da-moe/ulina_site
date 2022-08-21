use async_trait::async_trait;
use common::UserData;
use loader::{LoadHandler, LoadProps, Loader};
use util::BUTTON_CLASS;
use yew::prelude::*;
use yew_router::prelude::*;

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
            {back!()}
            if user.discord.is_some() || user.isAdmin{
                <a href="/logout" class={BUTTON_CLASS}>{"Logout"}</a>
            }
            else if user.discord.is_none(){
                <a href="/discord-login" class={BUTTON_CLASS}>{"Login"}</a>
            }

            <div>
                <Link<Route> to={Route::Map}>{"map"}</Link<Route>>
            </div>
            <div>
                <Link<Route> to={Route::Time}>{"time"}</Link<Route>>
            </div>
            <div>
                <Link<Route> to={Route::Nations}>{"nations"}</Link<Route>>
            </div>
            if user.isAdmin{
                <div>
                    <Link<Route> to={Route::Changes}>{"changes"}</Link<Route>>
                </div>
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
