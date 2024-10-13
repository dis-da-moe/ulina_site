use yew::{function_component, html};

#[function_component(Loading)]
pub fn rendered_at() -> Html {
    html! {
        <div style="height: 100%; width: 100%">
            <p>{"loading"}</p>
        </div>
    }
}
