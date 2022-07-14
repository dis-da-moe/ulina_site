use crate::route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

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
            </>
        }
    }
}
