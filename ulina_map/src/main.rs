mod error;
mod event_bus;
mod infobox;
mod loading;
mod map;
mod nation;
mod util;
mod backend;
mod flag;

use std::{rc::Rc, collections::HashMap};

use backend::start_load;
use common::{LoadMap, NationId, LOCAL_URL};
use infobox::Infobox;
use loading::Loading;
use map::{Map, NationNameId};
use error::Error;
use web_sys::{console, Element, Node};
use yew::prelude::*;
struct App {
    children: Option<Result<Vec<Html>, String>>
}

enum Msg {
    Loaded(Result<LoadMap, String>),
}


impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async move {
            Msg::Loaded(start_load().await)
        });

        Self {
            children: None
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        
        let response = match msg {
            Msg::Loaded(response) => response
        };

        let children = response.map(|load|{
            let ids = load.nations.iter().map(|nation| {
                NationNameId{
                    id: nation.core.nationId,
                    id_string: nation.core.nationId.to_string(),
                    name: nation.core.name.clone()
                }
            }).collect::<Vec<_>>();

            let map = html!{
                <Map nations={ids} map={load.map.file}/>
            };

            let mut nation_data = HashMap::new();

            for nation in load.nations {
                nation_data.insert(nation.core.nationId, nation);
            }

            let nation_data = Rc::new(nation_data);

            let infobox = html!{
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

fn main() {
    yew::start_app::<App>();
}
