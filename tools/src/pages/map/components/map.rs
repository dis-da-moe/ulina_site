use crate::util::viewbox::Viewbox;
use super::nation::{Nation, Rect};
use crate::util::by_id;

use glam::Vec2;
use gloo_events::EventListener;
use web_sys::{MouseEvent, WheelEvent};
use yew::{html, Component, Html, NodeRef, Properties};

use crate::util::get_vec;
use crate::util::XMLNS;

#[derive(Debug, PartialEq, Clone)]
pub struct NationNameId {
    pub id: i64,
    pub id_string: String,
    pub name: String,
}

#[derive(Clone)]
pub enum Msg {
    MouseHeld(bool),
    MouseMove(Vec2),
    MouseScroll(WheelEvent),
    Reset,
}

#[derive(Debug, Properties, PartialEq)]
pub struct MapProps {
    pub nations: Vec<NationNameId>,
    pub map: String,
}
pub struct Map {
    children: Vec<Html>,
    view: Viewbox,
    original_view: Viewbox,
    mouse_held: bool,
    svg: NodeRef,
    scale: f32,
    mouse_up_listener: Option<EventListener>,
}

impl Component for Map {
    type Message = Msg;

    type Properties = MapProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let nations = &ctx.props().nations;
        let map = ctx.props().map.clone();

        let div = gloo_utils::document().create_element("div").unwrap();
        div.set_inner_html(&map);

        let nation_elements = by_id(&div, "NATIONS".to_string()).unwrap();
        let children = nation_elements.get_elements_by_tag_name("g");

        let children: Vec<Html> = get_vec(&children)
            .into_iter()
            .map(|element| {
                let id = element.id();

                let nation_id = match nations.iter().position(|nation| nation.id_string == id) {
                    Some(index) => Some(nations.get(index)),
                    None => None,
                }
                .flatten()
                .cloned();
                let rect = element.get_bounding_client_rect();
                let rect = Rect {
                    size: Vec2 {
                        x: rect.width() as f32,
                        y: rect.height() as f32,
                    },
                    pos: Vec2 {
                        x: rect.x() as f32,
                        y: rect.y() as f32,
                    },
                };
                html! {
                    <Nation inner={element.inner_html()} nation={nation_id} rect={rect} dispatch={true}/>
                }
            })
            .collect();

        let view_box: Viewbox = div
            .first_element_child()
            .unwrap()
            .get_attribute("viewBox")
            .unwrap()
            .parse()
            .unwrap();

        Self {
            children,
            view: view_box.clone(),
            original_view: view_box,
            mouse_held: false,
            svg: NodeRef::default(),
            scale: 1.,
            mouse_up_listener: None,
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MouseHeld(held) => {
                self.mouse_held = held;
                false
            }
            Msg::MouseMove(delta) => {
                if self.mouse_held {
                    self.view.pos -= (delta * 5.) / self.scale;
                    true
                } else {
                    false
                }
            }
            Msg::MouseScroll(e) => {
                e.prevent_default();

                self.scale += e.delta_y() as f32 * 0.005;
                self.scale = self.scale.min(10.).max(1.);

                true
            }
            Msg::Reset => {
                self.scale = 1.;
                self.view = self.original_view.clone();

                true
            }
        }
    }

    fn rendered(&mut self, ctx: &yew::Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }

        let window = gloo_utils::window();
        let mouse_up = ctx.link().callback(|_| Msg::MouseHeld(false));

        let listener = EventListener::new(&window, "mouseup", move |_| {
            mouse_up.emit(());
        });

        self.mouse_up_listener = Some(listener);
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let onmousedown = ctx.link().callback(|_| Msg::MouseHeld(true));
        let mouse_move = ctx.link().callback(|e: MouseEvent| {
            Msg::MouseMove(Vec2 {
                x: e.movement_x() as f32,
                y: e.movement_y() as f32,
            })
        });
        let scroll = ctx.link().callback(|e| Msg::MouseScroll(e));

        let onreset = ctx.link().callback(|_| Msg::Reset);
        //TODO: pinch zoom in, finger drag pan

        html! {
            <>
            <svg ref={self.svg.clone()} xmlns={XMLNS}
                {onmousedown}
                onmousemove={mouse_move}
                onwheel={scroll}
                transform={format!("scale({})", self.scale)}
                viewBox={self.view.to_string()}
                class="border-dashed border-4 border-gray-400"
                >
                {for self.children.clone()}
            </svg>

            <div onclick={onreset} class="absolute unselectable bg-slate-700 text-white opacity-70 w-[10vh] grid place-items-center">
                <div class="m-1">
                    {"Reset"}
                </div>
            </div>

            </>
        }
    }
}
