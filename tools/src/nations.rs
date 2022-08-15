use async_trait::async_trait;
use common::{LoadNations, NationContinentId};

use crate::loader::{LoadHandler, Loader};
use crate::util::log;
use crate::{backend, Route};
use std::collections::HashMap;

use crate::loader::LoadProps;
use common::{NationContinent, CONTINENTS};
use web_sys::{HtmlInputElement, InputEvent};
use yew::{html, Component, Context, TargetCast};
use yew_router::prelude::Link;

#[async_trait(?Send)]
impl LoadHandler<(), LoadNations, LoadNations> for Loader<(), LoadNations, LoadNations, Nations> {
    async fn load(_props: ()) -> Result<LoadNations, String> {
        backend::load_nations().await
    }

    fn on_load(mut loaded: LoadNations) -> LoadNations {
        loaded.data.sort_by(|a, b| a.name.cmp(&b.name));
        loaded
    }
}

pub type App = Loader<(), LoadNations, LoadNations, Nations>;

#[derive(Clone, Copy)]
struct NationIndex(usize);

pub struct Nations {
    my_nation: Option<NationIndex>,
    searched_nations: Option<Vec<NationIndex>>,
    selected_continents: HashMap<&'static str, bool>,
}

pub enum Msg {
    NameSearch(String),
    Checkbox(&'static str, bool),
}

impl Component for Nations {
    type Message = Msg;

    type Properties = LoadProps<LoadNations>;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let loaded = &ctx.props().loaded;
        let my_nation = loaded
            .data
            .iter()
            .position(|nation| Some(&nation.ownerDiscord) == loaded.user.owner_discord.as_ref())
            .map(|index| NationIndex(index));

        let selected_continents = CONTINENTS
            .iter()
            .map(|continent| (*continent, true))
            .collect();

        Nations {
            my_nation,
            searched_nations: None,
            selected_continents,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NameSearch(name) => {
                let name = name.trim();

                self.searched_nations = if name.is_empty() {
                    None
                } else {
                    let name = name.to_lowercase();
                    Some(
                        ctx.props()
                            .loaded
                            .data
                            .iter()
                            .enumerate()
                            .filter_map(|(index, nation)| {
                                if nation.name.to_lowercase().contains(&name) {
                                    Some(NationIndex(index))
                                } else {
                                    None
                                }
                            })
                            .collect(),
                    )
                }
            }
            Msg::Checkbox(continent, checked) => {
                *self.selected_continents.get_mut(continent).unwrap() = checked;
            }
        }

        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let text_on_input = ctx.link().callback(|e: InputEvent| {
            Msg::NameSearch(e.target_dyn_into::<HtmlInputElement>().unwrap().value())
        });
        let checkbox_input = |continent| {
            ctx.link().callback(move |e: InputEvent| {
                Msg::Checkbox(
                    continent,
                    e.target_dyn_into::<HtmlInputElement>().unwrap().checked(),
                )
            })
        };

        let continents = self.selected_continents.iter().map(|(name, checked)| {
            html! {
                <div>
                    <input type="checkbox" oninput={checkbox_input(name)} checked={*checked}/>
                    <span>{name.clone()}</span>
                </div>
            }
        });

        let nations: Vec<_> = self
            .searched_nations
            .as_ref()
            .map(|nations| {
                nations
                    .iter()
                    .map(|index| get_nation(ctx, *index))
                    .collect()
            })
            .unwrap_or(ctx.props().loaded.data.iter().collect());

        let nations = nations.into_iter()
        .filter(|nation| *self.selected_continents.get(nation.continentName.as_str()).unwrap())
        .map(|nation|{
            html!{
                <div class="table-row">
                    <Link<Route> classes={"table-cell"} to={Route::Nation{id: nation.nationId}}>{&nation.name}</Link<Route>>
                    <div class="table-cell">{&nation.continentName}</div>
                </div>
            }
        });
        const BUTTON_CLASS: &str = "bg-blue-400 rounded-lg p-1";
        html! {
            <>
            <div class="flex flex-wrap">
                <span>{"Search: "}</span>
                <input placeholder="Enter nation name" oninput={text_on_input}/>
                <div class="dropdown relative">
                    <button class="inline-flex items-center">
                        <span class="mr-1">{"Continents"}</span>
                        <svg class="fill-current h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20"><path d="M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z"/>
                        </svg>
                    </button>
                    <div class="dropdown-menu absolute hidden text-gray-700 pt-1">
                        {for continents}
                    </div>
                </div>
                if let Some(index) = self.my_nation{
                    <Link<Route> classes={BUTTON_CLASS} to={Route::Nation{id: get_nation(ctx, index).nationId}}>{"My Nation"}</Link<Route>>
                }
                else{
                    <a class={BUTTON_CLASS} href="/discord-login">{"My Nation"}</a>
                }
            </div>

            <div class="table max-w-full w-full md:w-[50%]">
                <div class="table-header-group">
                    <div class="table-cell">{"Name"}</div>
                    <div class="table-cell">{"Continent"}</div>
                </div>
                <div class="table-row-group bg-gray-600 text-zinc-300">
                    {for nations}
                </div>
            </div>
            </>
        }
    }
}

fn get_nation(ctx: &Context<Nations>, index: NationIndex) -> &NationContinentId {
    ctx.props().loaded.data.get(index.0).unwrap()
}
