use yew::{classes, html, Callback, Component, Html, Properties};

use crate::{backend::url, util::EMPTY_DIV};

#[derive(PartialEq, Properties)]
pub struct FlagProps {
    pub flag: Option<String>,
    pub loaded: bool,
    pub on_load: Callback<()>,
}

pub struct Flag;

pub enum Msg {
    Loaded,
}

const CLASS: &str = ".object-scale-down max-h-[40vh] mx-auto p-3";

impl Component for Flag {
    type Message = Msg;

    type Properties = FlagProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Loaded => {
                ctx.props().on_load.emit(());
            }
        }
        false
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let onload = ctx.link().callback(|_| Msg::Loaded);

        match (&ctx.props().flag, &ctx.props().loaded) {
            (None, _) => {
                html! {
                    <div class={EMPTY_DIV}>
                        {"no flag"}
                    </div>
                }
            }
            (Some(flag), false) => {
                html! {
                    <>
                        <img class={classes!(CLASS)} src={url("loading.svg")}/>
                        <img class={classes!(CLASS)} src={flag.clone()} onload={onload} style="display:none"/>
                    </>
                }
            }
            (Some(flag), true) => {
                html! {
                    <img class={classes!(CLASS)} src={flag.clone()}/>
                }
            }
        }
    }

    fn changed(&mut self, _ctx: &yew::Context<Self>) -> bool {
        true
    }
}
