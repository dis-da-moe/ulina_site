use async_trait::async_trait;
use common::LoadNation;
use yew::prelude::*;

use crate::{
    backend::load_nation,
    loader::{LoadHandler, LoadProps, Loader}, show_nation::{show_info, show_trivia, field_title},
};

pub struct Nation{
    is_mine: bool,
    flag_loaded: bool
}

pub enum Msg{
    FlagLoaded
}

impl Component for Nation {
    type Message = Msg;

    type Properties = LoadProps<LoadNation>;

    fn create(ctx: &Context<Self>) -> Self {
        let loaded = &ctx.props().loaded;
        Nation { is_mine: loaded.user.owner_discord.as_ref() == Some(&loaded.data.core.ownerDiscord), flag_loaded: false }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FlagLoaded => self.flag_loaded = true
        };

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let flag_load = ctx.link().callback(|_| Msg::FlagLoaded);
        let nation = &ctx.props().loaded.data;
        html!{
            <div class="flex flex-col place-items-center">
                {field_title("Name", &nation.core.name)}
                {format!("is yours: {}", self.is_mine)}
                {show_info(nation, flag_load, self.flag_loaded)}
                {show_trivia(nation)}
            </div>
        }
    }
}

pub type App = Loader<Props, LoadNation, LoadNation, Nation>;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub id: i64,
}

#[async_trait(?Send)]
impl LoadHandler<Props, LoadNation, LoadNation> for App {
    async fn load(props: Props) -> Result<LoadNation, String> {
        load_nation(props.id).await
    }

    fn on_load(loaded: LoadNation) -> LoadNation {
        loaded
    }
}
