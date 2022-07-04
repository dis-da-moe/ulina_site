use common::{NationAll, NationId};
use web_sys::{Element, MouseEvent, Node};
use yew::{html, use_node_ref, Callback, Component, NodeRef, Properties};
use yew_agent::{Dispatched, Dispatcher};

use crate::{event_bus::{EventBus, Content}, map::NationNameId};

pub struct Nation {
    node_ref: NodeRef,
    bus: Dispatcher<EventBus>,
}

#[derive(Debug, PartialEq, Properties)]
pub struct NationProps {
    pub inner: String,
    pub nation: Option<NationNameId>,
}

pub enum Msg {
    Clicked(MouseEvent),
}

impl Component for Nation {
    type Message = Msg;

    type Properties = NationProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        let node_ref = NodeRef::default();
        let bus = EventBus::dispatcher();

        Nation { node_ref, bus }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let onclick = ctx.link().callback(|e: MouseEvent| Msg::Clicked(e));

        html! {
            <g ref={self.node_ref.clone()} {onclick}>
            </g>
        }
    }

    fn rendered(&mut self, ctx: &yew::Context<Self>, first_render: bool) {
        if first_render {
            let element: Element = self.node_ref.cast().unwrap();
            element.set_inner_html(&ctx.props().inner);
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked(_e) => {
                web_sys::console::log_1(&format!("{:?}", ctx.props().nation).into());


                self.bus.send(ctx.props().nation.as_ref().map(|nation| nation.id));
            }
        }
        false
    }
}
