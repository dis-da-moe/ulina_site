use common::NationId;
use yew::prelude::*;
use super::LinkButton;

#[derive(Properties, PartialEq)]
pub struct MyNationProps{
    pub nation: Option<NationId>
}

#[function_component(MyNation)]
pub fn my_nation(props: &MyNationProps) -> Html{
    let link = if let Some(id) = props.nation{
        format!("/tools/nation/{}", id.0)
    }
    else{
        "/discord-login".to_string()
    };

    html!{
        <LinkButton text="My Nation" {link}/>
    }
}