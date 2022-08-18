use yew::prelude::*;
use yew_router::prelude::{use_history, History};

#[function_component(Back)]
pub fn back() -> Html {
    match use_history(){
        Some(history) => {
            let onclick = Callback::from(move |e: MouseEvent| {
                e.prevent_default();
                history.back();
            });

            html! {
                <>
                    <a {onclick} href="" class="underline">{"back"}</a>
                </>
            }
        }
        _ => html!{}
    }    
}

#[macro_export]
macro_rules! back {
    () => {
        html!(<crate::components::Back/>)
    };
}