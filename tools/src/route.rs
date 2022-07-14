use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/tools")]
    Home,
    #[at("/tools/map")]
    Map,
    #[at("/tools/time")]
    Time,
}
