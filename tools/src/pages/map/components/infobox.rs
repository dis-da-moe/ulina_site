use std::collections::HashMap;
use std::rc::Rc;

use crate::util::XMLNS;
use common::NationAll;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::{
    display::{show_info, show_trivia, title},
    pages::map::event_bus::{Content, EventBus},
};

pub enum Msg {
    Update(Content),
    FlagLoad(i64),
    ChangeTab(Tab),
}

#[derive(PartialEq, Properties)]
pub struct InfoBoxProps {
    pub nation_data: Rc<HashMap<i64, NationAll>>,
}

#[derive(Clone, Copy)]
pub enum Tab {
    Info,
    Trivia,
}

pub struct Infobox {
    current_nation: Option<i64>,
    loaded_flags: Vec<i64>,
    current_tab: Tab,
    _producer: Box<dyn Bridge<EventBus>>,
}

impl Infobox {
    fn tabs(&self, ctx: &Context<Self>) -> Html {
        const SELECTED_TAB: &str = "fill-[#f7f7e9]";
        const UNSELECTED_TAB: &str = "fill-[#abafb4]";

        let tab_click = |tab| ctx.link().callback(move |_| Msg::ChangeTab(tab));

        let (info_colour, trivia_colour) = match self.current_tab {
            Tab::Info => (SELECTED_TAB, UNSELECTED_TAB),
            Tab::Trivia => (UNSELECTED_TAB, SELECTED_TAB),
        };

        html! {
            <div class="col-span-2">
            <svg class="max-h-8" xmlns={XMLNS} viewBox="0 0 180 48">
                <g onclick={tab_click(Tab::Info)}>
                    <polygon  class={info_colour} points="0 0 0 50 90 50 86 0 0 0" />
                    <text class="cls2-2" transform="translate(15.07 36.63)">
                        {"Info"}
                    </text>
                </g>

                <g onclick={tab_click(Tab::Trivia)}>
                    <polygon class={trivia_colour} points="86 0 90 50 180 50 176 0 86 0" />
                    <text class="cls2-10" transform="translate(95.03 35.73) scale(0.96 1)">
                        {"Trivia"}
                    </text>
                </g>
            </svg>
        </div>
        }
    }

    fn flag_loaded(&self, id: i64) -> bool {
        self.loaded_flags.iter().find(|x| **x == id).is_some()
    }
}

impl Component for Infobox {
    type Message = Msg;
    type Properties = InfoBoxProps;

    fn create(ctx: &Context<Self>) -> Self {
        Infobox {
            current_nation: None,
            loaded_flags: vec![],
            current_tab: Tab::Info,
            _producer: EventBus::bridge(ctx.link().callback(Msg::Update)),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let current_nation = self
            .current_nation
            .map(|id| ctx.props().nation_data.get(&id))
            .flatten();

        let content = match current_nation {
            None => {
                return html! {};
            }
            Some(nation) => {
                let main = match self.current_tab {
                    Tab::Info => {
                        let id = nation.core.nationId;
                        let flag_load = ctx.link().callback(move |_| Msg::FlagLoad(id));
                        let loaded = self.flag_loaded(id);

                        show_info(&nation, flag_load, loaded)
                    }
                    Tab::Trivia => show_trivia(&nation),
                };
                html! {
                    <>
                        {title(&nation.core.name)}
                        {main}
                    </>
                }
            }
        };

        let close = ctx.link().callback(|_| Msg::Update(None));

        let tabs = self.tabs(ctx);

        html! {
            <div class="fixed top-0 right-0 mr-1 mt-1 w-1/2 bg-gray-500 p-1 sm:w-1/3 md:w-1/4 md:text-lg">

                <div class="unselectable grid grid-cols-4">
                    {tabs}
                    <div onclick={close} class="absolute right-1 top-1 h-5 w-5 rounded-lg bg-red-400 text-center text-sm text-white">
                        {"X"}
                    </div>
                </div>

                <div class="overflow-y-auto overflow-x-hidden bg-[#f7f7e9] p-1 max-h-[90vh]">
                <div class="space-y-0.5 border-x-2 border-b-2 border-solid border-gray-400 bg-[#f7f7e9] p-1 text-black">
                  {content}
                </div>
              </div>

            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update(request) => {
                if request == self.current_nation {
                    false
                } else {
                    self.current_nation = request;
                    self.current_tab = Tab::Info;
                    true
                }
            }
            Msg::FlagLoad(id) => {
                self.loaded_flags.push(id);
                true
            }
            Msg::ChangeTab(tab) => {
                self.current_tab = tab;
                true
            }
        }
    }
}
