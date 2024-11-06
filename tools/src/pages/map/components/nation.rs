use glam::Vec2;
use web_sys::{Element, MouseEvent};
use yew::{html, Component, NodeRef, Properties};
use yew_agent::{Dispatched, Dispatcher};

use super::super::event_bus::EventBus;

use super::map::NationNameId;

pub struct Nation {
    node_ref: NodeRef,
    bus: Option<Dispatcher<EventBus>>,
    stroke: (String, usize),
}

#[derive(Debug, PartialEq)]
pub struct Rect {
    pub size: Vec2,
    pub pos: Vec2,
}

#[derive(Debug, PartialEq, Properties)]
pub struct NationProps {
    pub inner: String,
    pub nation: Option<NationNameId>,
    pub rect: Rect,
    pub dispatch: bool,
}

pub enum Msg {
    Clicked(MouseEvent),
    Hovered(bool),
}

impl Component for Nation {
    type Message = Msg;

    type Properties = NationProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let node_ref = NodeRef::default();
        let bus = if ctx.props().dispatch {
            Some(EventBus::dispatcher())
        } else {
            None
        };

        Nation {
            node_ref,
            bus,
            stroke: ("black".to_string(), 5),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let onclick = ctx.link().callback(|e: MouseEvent| Msg::Clicked(e));
        let onmouseenter = ctx.link().callback(|_| Msg::Hovered(true));
        let onmouseleave = ctx.link().callback(|_| Msg::Hovered(false));

        html! {
            <g ref={self.node_ref.clone()} {onclick} {onmouseenter} {onmouseleave} style={format!("stroke:{};stroke-width:{};", self.stroke.0, self.stroke.1)}>
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
                if let Some(bus) = self.bus.as_mut() {
                    bus.send(ctx.props().nation.as_ref().map(|nation| nation.id));
                }
                false
            }
            Msg::Hovered(hover) => {
                if ctx.props().nation.is_some() {
                    if hover {
                        self.stroke = ("white".to_string(), 10);
                        let current = self.node_ref.cast::<Element>().unwrap();
                        let parent = current.parent_element().unwrap();
                        parent.insert_before(&current, None).unwrap();
                    } else {
                        self.stroke = ("black".to_string(), 5);
                    }

                    true
                } else {
                    false
                }
            }
        }
    }
}
