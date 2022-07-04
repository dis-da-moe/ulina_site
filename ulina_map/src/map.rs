use std::rc::Rc;

use crate::{nation::Nation, util::{by_id, log}};
use common::LoadMap;
use web_sys::Node;
use yew::{html, html_nested, Children, Component, Html, Properties};

use crate::util::get_vec;

#[derive(Debug, PartialEq, Clone)]
pub struct NationNameId{
    pub id: i64,
    pub id_string: String,
    pub name: String
}

#[derive(Debug, Properties, PartialEq)]
pub struct MapProps {
    pub nations: Vec<NationNameId>,
    pub map: String
}
pub struct Map {
    children: Vec<Html>,
    view_box: String,
}

impl Component for Map {
    type Message = ();

    type Properties = MapProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let nations = &ctx.props().nations;
        let map = ctx.props().map.clone();

        let div = gloo_utils::document().create_element("div").unwrap();
        log(map.clone());
        div.set_inner_html(&map);

        let nation_elements = by_id(&div, "NATIONS".to_string()).unwrap();
        let children = nation_elements.get_elements_by_tag_name("g");

        let children: Vec<Html> = get_vec(&children)
            .into_iter()
            .map(|element| {
                let id = element.id();
                
                let nation_id = match 
                    nations
                    .iter()
                    .position(|nation| nation.id_string == id)
                {
                    Some(index) => Some(nations.get(index)),
                    None => None,
                }.flatten().cloned();

                html! {
                    <Nation inner={element.inner_html()} nation={nation_id}/>
                }
            })
            .collect();
        
        /* 
        let style: Node = div
            .get_elements_by_tag_name("defs")
            .get_with_index(0)
            .unwrap()
            .into();
        */

        //children.insert(0, Html::VRef(style));

        let view_box = div
            .first_element_child()
            .unwrap()
            .get_attribute("viewBox")
            .unwrap();

        Self { children, view_box }
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <div class="max-w-screen m-auto max-h-screen object-contain">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox={self.view_box.clone()}>
                {for self.children.clone()}
            </svg>
            </div>
        }
    }
}
