use yew::prelude::*;
use crate::Route;
use yew_router::prelude::Link;

const BUTTON_CLASS: &str = "btn btn-primary text-center d-md-flex justify-content-md-center w-fit h-fit align-items-md-center text-[13px]";

pub trait MakeButton{
    fn to_button(self, inner: Html) -> Html;
}

impl MakeButton for &str{
    fn to_button(self, inner: Html) -> Html {
        html!{
            <a class="a-hidden" href={self.to_string()}>
                {inner}
            </a>
        }
    }
}
impl MakeButton for Callback<MouseEvent>{
    fn to_button(self, inner: Html) -> Html {
        html!{
            <a class="a-hidden" onclick={self}>
                {inner}
            </a>
        }
    }
}
impl MakeButton for Route{
    fn to_button(self, inner: Html) -> Html {
        html!{
            <Link<Route> to={self}>
                {inner}
            </Link<Route>>
        }
    }
}

pub fn button(link: impl MakeButton, text: impl ToString) -> Html{
    let inner = html!{
        <button class={BUTTON_CLASS} type="button">
                {text.to_string()}
        </button>
    };
    link.to_button(inner)
}
