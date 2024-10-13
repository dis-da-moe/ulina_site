use async_trait::async_trait;
use common::{LoadNations, NationContinentId};

use crate::loader::{LoadHandler, Loader};

use crate::components::MyNation;
use crate::loader::LoadProps;
use crate::util::{input_checkbox, input_text};
use crate::{backend, navbar, Route};
use common::Id;
use common::CONTINENTS;
use std::collections::HashMap;
use web_sys::{InputEvent};
use yew::{html, Component, Context, Html};
use yew_router::prelude::Link;
#[async_trait(?Send)]
impl LoadHandler<LoadNations> for Loader<LoadNations, Nations> {
    async fn load() -> Result<LoadNations, String> {
        backend::load_nations().await
    }

    fn on_load(mut loaded: LoadNations) -> LoadNations {
        loaded.data.sort_by(|a, b| a.name.cmp(&b.name));
        if !loaded.user.isAdmin {
            loaded.data = loaded
                .data
                .into_iter()
                .filter(|nation| !nation.removed)
                .collect();
        } else {
            loaded.data.sort_by(|a, b| a.removed.cmp(&b.removed));
        }
        loaded
    }
}

pub type App = Loader<LoadNations, Nations>;

#[derive(Clone, Copy)]
struct NationIndex(usize);

pub struct Nations {
    my_nation: Option<NationIndex>,
    searched_nations: Option<Vec<NationIndex>>,
    include_removed: bool,
    selected_continents: HashMap<&'static str, bool>,
}

pub enum Msg {
    NameSearch(String),
    Checkbox(&'static str, bool),
    Removed(bool),
}
const COLUMN_CLASS: &str = "col d-flex justify-content-center align-items-center mb-1";

impl Component for Nations {
    type Message = Msg;

    type Properties = LoadProps<LoadNations>;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let loaded = &ctx.props().loaded;
        let my_nation = loaded
            .data
            .iter()
            .position(|nation| Some(&nation.ownerDiscord) == loaded.user.discord.as_ref())
            .map(|index| NationIndex(index));

        let selected_continents = CONTINENTS
            .iter()
            .map(|continent| (*continent, true))
            .collect();

        Nations {
            my_nation,
            searched_nations: None,
            selected_continents,
            include_removed: false,
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
            Msg::Removed(removed) => {
                self.include_removed = removed;
            }
        }

        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
        <>
        {navbar!()}
        <div class="vstack">

            <div class="row d-flex justify-content-center align-items-center align-content-center me-auto" style="margin-left: 0;">
                <span>{"Search: "}</span>

                <div class="col d-flex d-lg-flex d-xl-flex justify-content-center align-items-center align-items-lg-center justify-content-xl-center" style="max-width: 100%;margin-bottom: 5px;">
                    <input class="d-grid justify-content-center align-items-md-center" style="margin-bottom: 0px;max-height: 34px; color:black;"  id="name"
                        type="text" placeholder="Enter nation name here" oninput={ctx.link().callback(|e: InputEvent| Msg::NameSearch(input_text(e)))}
                    />
                </div>

                {self.dropdown(ctx)}

                {self.removed(ctx)}

                <div class={COLUMN_CLASS}>
                    <MyNation nation={self.my_nation.map(|index| get_nation(ctx, index).id())}/>
                </div>
            </div>

            {self.table(ctx)}

        </div>
        </>
        }
    }
}

impl Nations {
    fn table(&self, ctx: &Context<Self>) -> Html {
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
        .filter(|nation| nation.removed == self.include_removed)
        .map(|nation|{
            let name = if nation.removed{
                format!("{} [removed]", nation.name, )
            }else{
                nation.name.clone()
            };

            html!{
                <tr>
                    <td>
                        <Link<Route> to={Route::Nation{id: nation.nationId}}>{name}</Link<Route>>
                    </td>
                    <td>{&nation.continentName}</td>
                </tr>
            }
        });

        html! {
            <div class="table-responsive" style="margin-top: 13px;">
                <table class="table table-striped table-hover table-dark table-sm">
                    <thead>
                        <tr>
                            <th>{"Name"}</th>
                            <th>{"Continent"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {for nations}
                    </tbody>
                </table>
            </div>
        }
    }

    fn removed(&self, ctx: &Context<Self>) -> Html {
        let removed_callback = ctx.link().callback(|e| Msg::Removed(input_checkbox(e)));

        html! {
            if ctx.props().loaded.user.isAdmin{
                <div class="col">
                    <div class="form-check">
                        <input class="form-check-input" type="checkbox" id="removed" checked={self.include_removed} oninput={removed_callback}/>
                        <label class="form-check-label" for="removed">{"Removed"}</label>
                    </div>
                </div>
            }
        }
    }

    fn dropdown(&self, ctx: &Context<Self>) -> Html {
        let checkbox_input = |continent| {
            ctx.link()
                .callback(move |e: InputEvent| Msg::Checkbox(continent, input_checkbox(e)))
        };

        let continents = self.selected_continents.iter().map(|(&name, &checked)| {
            html! {
                <div class="form-check">
                    <input class="form-check-input" type="checkbox" id={name} oninput={checkbox_input(name)} checked={checked}/>
                    <label class="form-check-label" for={name}>{name}</label>
                </div>
            }
        });

        html! {
            <div class="col d-flex d-md-flex justify-content-center align-items-md-center" style="margin-bottom: 5px;">
                <div class="dropdown justify-content-center align-self-center">
                    <button class="btn btn-primary dropdown-toggle align-self-center" aria-expanded="false" data-bs-toggle="dropdown" type="button">
                        {"Continents"}
                    </button>
                    <div class="dropdown-menu" style="margin-right: 0;margin-left: 0;min-width: 0;padding-right: 19px;padding-left: 19px;">
                        {for continents}
                    </div>
                </div>
            </div>
        }
    }
}

fn get_nation(ctx: &Context<Nations>, index: NationIndex) -> &NationContinentId {
    ctx.props().loaded.data.get(index.0).unwrap()
}
