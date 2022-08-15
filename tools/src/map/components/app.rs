use std::{collections::HashMap, rc::Rc};

use super::{
    infobox::Infobox,
    map::{Map, NationNameId},
};
use crate::backend::load_map;
use crate::error::Error;
use crate::loading::Loading;
use common::LoadMap;
use yew::prelude::*;

pub struct App {
    children: Option<Result<Vec<Html>, String>>,
}

pub enum Msg {
    Loaded(Result<LoadMap, String>),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link()
            .send_future(async move { Msg::Loaded(load_map().await) });

        Self { children: None }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let response = match msg {
            Msg::Loaded(response) => response,
        };

        let children = response.map(|load| {
            let ids = load
                .nations
                .iter()
                .map(|nation| NationNameId {
                    id: nation.core.nationId,
                    id_string: nation.core.nationId.to_string(),
                    name: nation.core.name.clone(),
                })
                .collect::<Vec<_>>();

            let map = html! {
                <Map nations={ids} map={load.map.file}/>
            };

            let mut nation_data = HashMap::new();

            for nation in load.nations {
                nation_data.insert(nation.core.nationId, nation);
            }

            let nation_data = Rc::new(nation_data);

            let infobox = html! {
                <Infobox nation_data={nation_data}/>
            };

            vec![map, infobox]
        });

        self.children = Some(children);

        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match &self.children {
            None => {
                return html! {
                    <Loading/>
                };
            }
            Some(Err(error)) => {
                return html! {
                    <Error error_message={error.clone()}/>
                };
            }
            Some(Ok(children)) => {
                html! {
                    <div class="flex h-screen w-screen overflow-hidden bg-[#c5e1ef]">

                    {for children.clone()}

                    </div>
                }
            }
        }
    }
}
