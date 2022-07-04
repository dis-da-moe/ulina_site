use std::collections::HashMap;
use std::rc::Rc;

use common::{NationAll};
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};
use crate::flag::Flag;

use crate::{
    event_bus::{EventBus, Content}, util::log
};

pub enum Msg {
    Update(Content),
    FlagLoad(i64)
}

#[derive(PartialEq, Properties)]
pub struct InfoBoxProps{
    pub nation_data: Rc<HashMap<i64, NationAll>>
}

pub struct Infobox {
    current_nation: Option<i64>,
    loaded_flags: Vec<i64>,
    _producer: Box<dyn Bridge<EventBus>>,
}

fn title(title: &str) -> Html{
    html!{
        <span class="infobox-title">
            {title}
        </span>
    }
}

fn field(content: &str) -> Html{
    html!{
        <span class="infobox-field">
            {content}
        </span>
    }
}

fn info(message: Html) -> Html{
    html!{
        <div class="flex justify-center" style="height:33vh">
            {message}
        </div>
    }
}

impl Infobox{
    fn show_box(self: &Self, ctx: &Context<Self>, nation: &NationAll) -> Html{
        let socials = nation.socials.iter().map(|social| html!{
            <>
            {title(&social.platform)}
            <span class="infobox-field">
                <a class="text-blue-600 visited:text-purple-600" href={social.link.clone()}> 
                    {&social.link}
                </a>
            </span>
            </>
        });

        let id = nation.core.nationId;

        let flag_load = ctx.link().callback(move |_| Msg::FlagLoad(id));

        html!{
            <>
                {title(&nation.core.name)}
    
                <Flag flag={nation.flag_link.clone()} on_load={flag_load} loaded={self.flag_loaded(nation.core.nationId)}/>
    
                {field_title("Continent", &nation.core.continentName)}
    
                if let Some(description) = nation.core.description.as_ref(){
                    {field_title("Description", &description)}
                }
    
                {for socials}
            </>
        }
    }

    fn flag_loaded(&self, id: i64) -> bool{
        self.loaded_flags.iter().find(|x| **x == id).is_some()
    }
}

fn field_title(title: &str, content: &str) -> Html{
    html!{
        <>
        <span class="infobox-title">
            {title}
        </span>
        <span class="infobox-field">
            {content}
        </span>
        </>
    }
}

impl Component for Infobox {
    type Message = Msg;
    type Properties = InfoBoxProps;

    fn create(ctx: &Context<Self>) -> Self {
        Infobox {
            current_nation: None,
            loaded_flags: vec![],
            _producer: EventBus::bridge(ctx.link().callback(Msg::Update)),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log(format!("{:?}", self.current_nation));

        let current_nation = self.current_nation.map(|id| ctx.props().nation_data.get(&id)).flatten();
        log(format!("{:?}", current_nation));

        let content = match current_nation  {
            None => {
                return html! {};
            }
            Some(nation) => {
                self.show_box(ctx, &nation)
                /*
                match ctx.props().nation_data.get(&id){
                    None => {
                        info(html!{
                            <div class="text-4xl">{"Loading..."}</div>
                        })
                    },
                    None => {
                        info(html!{
                            <Error error_message={"Can not find nation"}/>
                        })
                    },
                    Some(nation) => show_box(nation)
                }
                */
            }
        };
        
        let close = ctx.link().callback(|_| Msg::Update(None));

        html!{
            <div class="fixed top-0 right-0 mr-1 mt-1 w-1/2 bg-gray-500 p-1 sm:w-1/3 md:w-1/4 md:text-lg">
                
                <div class="unselectable grid grid-cols-4">
                    <div class="rounded-tl-sm bg-[#f7f7e9] indent-1 text-base">
                        {"Info"}
                    </div>
                    <div class="bg-gray-400 indent-1 text-base">
                        {"Trivia"}
                    </div>
                    <div onclick={close} class="absolute right-1 top-1 h-5 w-5 rounded-lg bg-red-400 text-center text-sm text-white">
                        {"X"}
                    </div>
                </div>

                <div class="bg-[#f7f7e9] p-1">
                <div class="space-y-0.5 border-x-2 border-b-2 border-solid border-gray-400 bg-[#f7f7e9] p-1">
                  {content}
                </div>
              </div>

            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update(request) => {
                if request == self.current_nation{
                    false
                }
                else{
                    self.current_nation = request;
                    true
                }
            },
            Msg::FlagLoad(id) => {
                self.loaded_flags.push(id);
                true
            }
        }
    }
}
