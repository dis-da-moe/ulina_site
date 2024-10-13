use crate::{Route};

use super::button::button;
use common::NationId;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MyNationProps {
    pub nation: Option<NationId>,
}

#[function_component(MyNation)]
pub fn my_nation(props: &MyNationProps) -> Html {
    if let Some(NationId(id)) = props.nation {
        button(Route::Nation { id }, "My Nation")
    } else {
        button("/discord-login", "Login")
    }
}
