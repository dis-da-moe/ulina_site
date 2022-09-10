use web_sys::Element;
use yew::prelude::*;

pub struct CreateNation {
    node_ref: NodeRef,
}

#[derive(PartialEq, Properties)]
pub struct CreateNationProps {
    pub inner: String,
    pub id: Option<String>,
    pub onclick: Option<Callback<MouseEvent>>,
    pub assigned: bool
}

impl Component for CreateNation {
    type Message = ();

    type Properties = CreateNationProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.node_ref
                .cast::<Element>()
                .unwrap()
                .set_inner_html(&ctx.props().inner);
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let id = ctx.props().id.clone().unwrap_or_else(||"".to_string());
        let assigned = ctx.props().assigned;
        let onclick = match ctx.props().onclick.clone(){
            Some(callback) if !assigned => callback,
            _ => ctx.link().callback(|_| ())
        };
        let fill = if assigned {"0.5"} else {"inhert"};
        html! {
            <g ref={self.node_ref.clone()} {onclick} id={id} style={format!("opacity:{}", fill)}>
            </g>
        }
    }
}
