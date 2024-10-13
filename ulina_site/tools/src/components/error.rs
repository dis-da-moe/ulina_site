use yew::virtual_dom::AttrValue;
use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct ErrorProps {
    pub error_message: AttrValue,
}

#[function_component(Error)]
pub fn rendered_at(props: &ErrorProps) -> Html {
    html! {
        <div style="width: 100%; height: 100%">
            <p>
                <b>{ "An error was encountered:" }</b>
                { &props.error_message }
            </p>
        </div>
    }
}
