use yew::prelude::*;
use crate::util::input_field;

#[function_component(App)]
pub fn app() -> Html{
    html!{
        <form action="/create-nation" method="POST" enctype="multipart/form-data">
            {input_field("name", "", false, true)}
            {input_field("continentName", "", false, true)}
            {input_field("ownerDiscord", "", false, true)}
            <input type="submit"/>
        </form>
    }
}