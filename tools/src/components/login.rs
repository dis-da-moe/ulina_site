use common::UserData;
use yew::prelude::*;

use super::button;

#[derive(Properties, PartialEq)]
pub struct LoginProps {
    pub user: UserData,
}

#[function_component(Login)]
pub fn login(props: &LoginProps) -> Html {
    let (text, link) = if props.user.discord.is_some() || props.user.isAdmin {
        ("Logout", "/logout")
    } else {
        ("Login", "/discord-login")
    };
    button(link, text)
}
