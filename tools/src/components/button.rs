use yew::prelude::*;

const BUTTON_CLASS: &str = "btn btn-primary text-center d-md-flex justify-content-md-center w-fit h-fit align-items-md-center text-[13px]";

macro_rules! button_props {
    ($name: tt, $data_name: tt, $data_type: ty) => {
        #[derive(Properties, PartialEq)]
        pub struct $name {
            pub text: String,
            pub $data_name: $data_type,
        }
    };
}

button_props!(LinkButtonProps, link, String);
button_props!(CallbackButtonProps, callback, Callback<MouseEvent>);

#[function_component(LinkButton)]
pub fn link_button(props: &LinkButtonProps) -> Html {
    html! {
        <a class="a-hidden" href={props.link.clone()}>
            <button class={BUTTON_CLASS} type="button">
                {props.text.clone()}
            </button>
        </a>
    }
}

#[function_component(CallbackButton)]
pub fn callback_button(props: &CallbackButtonProps) -> Html {
    html! {
        <a class="a-hidden" onclick={props.callback.clone()}>
            <button class={BUTTON_CLASS} type="button">
                {props.text.clone()}
            </button>
        </a>
    }
}
