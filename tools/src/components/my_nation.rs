use super::LinkButton;
use common::NationId;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MyNationProps {
    pub nation: Option<NationId>,
}

#[function_component(MyNation)]
pub fn my_nation(props: &MyNationProps) -> Html {
    let (link, text) = if let Some(id) = props.nation {
        (format!("/tools/nation/{}", id.0), "My Nation")
    } else {
        ("/discord-login".to_string(), "Login")
    };

    html! {
        <LinkButton text={text} {link}/>
    }
}
