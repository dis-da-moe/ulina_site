use std::{collections::HashMap, rc::Rc};

use super::{
    infobox::Infobox,
    map::{Map, NationNameId},
};
use crate::{
    backend::load_map,
    loader::{LoadProcessHandler, LoadProps, LoaderProcessor},
};
use async_trait::async_trait;
use common::LoadMap;
use yew::prelude::*;

type AppProps = LoadProps<Data>;
type Data = Vec<Html>;
pub type App = LoaderProcessor<(), LoadMap, Data, Root>;

#[function_component(Root)]
pub fn root(props: &AppProps) -> Html {
    html! {
        <>
        <div class="flex h-screen w-screen overflow-hidden bg-[#c5e1ef]">

        {for props.loaded.clone()}

        </div>
        </>
    }
}

#[async_trait(?Send)]
impl LoadProcessHandler<(), LoadMap, Data> for App {
    async fn load(_: ()) -> Result<LoadMap, String> {
        load_map().await
    }

    fn on_load(load: LoadMap) -> Data {
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
    }
}
