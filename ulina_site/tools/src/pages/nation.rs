use std::collections::HashMap;
use crate::components::button;
use crate::components::Flag;
use crate::navbar;
use async_trait::async_trait;
use common::{AddSocial, LoadNation, CONTINENTS};
use web_sys::{HtmlFormElement, HtmlInputElement};
use yew::prelude::*;
use crate::util::{input_field, INPUT_CONTAINER};

use crate::util::{input_checkbox, input_text};
use crate::{
    backend::load_nation,
    display::{field_title, show_info, show_trivia},
    loader::{LoadProcessHandler, LoadProps, LoaderProcessor},
};

pub struct Nation {
    is_mine: bool,
    logged_in: bool,
    flag_loaded: bool,
    optional_fields: HashMap<&'static str, Option<String>>,
    socials: Vec<AddSocial>,
    form: NodeRef,
    socials_field: NodeRef,
    continent_field: NodeRef,
    removed: bool,
    message: Option<String>,
    edit: bool,
}

type EditField = fn(&mut AddSocial) -> &mut String;
#[derive(Clone, Copy)]
pub struct SocialIndex(usize);
pub enum Msg {
    FlagLoaded,
    OptionalField(&'static str, String),
    Submit,
    AddSocial,
    RemoveSocial(SocialIndex),
    EditSocial(SocialIndex, String, EditField),
    Removed(bool),
    Edit,
}

impl Component for Nation {
    type Message = Msg;

    type Properties = LoadProps<LoadNation>;

    fn create(ctx: &Context<Self>) -> Self {
        let loaded = &ctx.props().loaded;
        let nation = &loaded.data.core;
        let optional_fields = HashMap::from([
            ("description", nation.description.clone()),
            ("leader", nation.leader.clone()),
            ("capital", nation.capital.clone()),
            ("ideology", nation.ideology.clone()),
            ("alliances", nation.ideology.clone()),
        ]);

        let socials = loaded
            .data
            .socials
            .iter()
            .map(|social| AddSocial {
                socials_id: Some(social.socialsId),
                link: social.link.clone(),
                platform: social.platform.clone(),
            })
            .collect();

        Nation {
            is_mine: loaded.user.discord.as_ref() == Some(&loaded.data.core.ownerDiscord),
            logged_in: loaded.user.discord.is_some(),
            flag_loaded: false,
            optional_fields,
            socials,
            form: NodeRef::default(),
            socials_field: NodeRef::default(),
            continent_field: NodeRef::default(),
            removed: nation.removed,
            message: None,
            edit: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FlagLoaded => self.flag_loaded = true,
            Msg::OptionalField(name, value) => {
                *self.optional_fields.get_mut(name).unwrap() = if value.trim().is_empty() {
                    None
                } else {
                    Some(value)
                };
            }
            Msg::AddSocial => {
                self.socials.push(AddSocial {
                    socials_id: None,
                    link: "".to_string(),
                    platform: "".to_string(),
                });
            }
            Msg::RemoveSocial(SocialIndex(index)) => {
                self.socials.remove(index);
            }
            Msg::EditSocial(index, value, editor) => {
                *editor(self.socials.get_mut(index.0).unwrap()) = value
            }
            Msg::Submit => {
                if let Some(continent_field) = &self.continent_field.cast::<HtmlInputElement>() {
                    if !CONTINENTS.contains(&continent_field.value().as_str()) {
                        self.message = Some("Invalid continent name".to_string());
                        return true;
                    }
                }

                let form = self.form.cast::<HtmlFormElement>().unwrap();

                if !form.check_validity() {
                    self.message = Some("Invalid form".to_string());
                    return true;
                }

                self.socials.retain(|social| {
                    !social.link.trim().is_empty() && !social.platform.trim().is_empty()
                });

                self.socials_field
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .set_value(&serde_json::to_string(&self.socials).unwrap());

                form.submit().unwrap();
            }
            Msg::Removed(removed) => {
                self.removed = removed;
            }
            Msg::Edit => {
                self.edit = true;
            }
        };

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let flag_load = ctx.link().callback(|_| Msg::FlagLoaded);
        let nation = &ctx.props().loaded.data;
        let is_admin = ctx.props().loaded.user.isAdmin;

        if self.edit {
            let on_field_input = |name| {
                ctx.link()
                    .callback(move |e| Msg::OptionalField(name, input_text(e)))
            };

            let optional_fields = self.optional_fields.iter()
                .map(|(name, value)| {
                    let oninput = on_field_input(*name);
                    html!{
                        <div class={INPUT_CONTAINER}>
                        <label>{name.to_string()}</label>
                        <textarea class="text-input" type="text" name={name.to_string()} value={value.clone().unwrap_or("".to_string())} {oninput}/>
                        </div>
                    }
                });

            let edit_social = |index, editor| {
                ctx.link()
                    .callback(move |e| Msg::EditSocial(index, input_text(e), editor))
            };

            let socials = self.socials.iter().enumerate().map(|(index, social)|{
                let edit = |editor| edit_social(SocialIndex(index), editor);

                html!{
                    <tr>
                        <td><input class="text-input" value={social.platform.clone()} oninput={edit(|social| &mut social.platform)}/></td>
                        <td><input class="text-input" value={social.link.clone()} oninput={edit(|social| &mut social.link)}/></td>
                        <td>{button(ctx.link().callback(move |_| Msg::RemoveSocial(SocialIndex(index))), "Delete")}</td>
                    </tr>
                }
            });

            html! {
                <>
                {navbar!()}

                <div class="pl-3 pt-2 mb-5">
                if let Some(message) = self.message.clone(){
                    <p>{message}</p>
                }

                <form ref={self.form.clone()} action="/edit-nation" method="POST" enctype="multipart/form-data" class="grid space-y-5">
                    {input_field("id", nation.core.nationId, true, true)}
                    <input type="text" name="socials" class="text-input" value="" hidden=true ref={self.socials_field.clone()}/>

                    {input_field("name", &nation.core.name, false, true)}
                    {for optional_fields}

                    <div>
                        <Flag flag={nation.flag_link.clone()} on_load={flag_load} loaded={self.flag_loaded}/>
                        <label>{"flag"}</label>
                        <input type="file" required=false name="flag" accept="image/png, image/jpeg"/>
                    </div>

                    if is_admin{
                        <div>
                        <label>{"Removed"}</label>
                        <input type="checkbox" required=false class="text-input" name="removed" checked={self.removed} oninput={ctx.link().callback(|e| Msg::Removed(input_checkbox(e)))}/>
                        </div>

                        {input_field("discord", &nation.core.ownerDiscord, false, true)}
                        <div>
                        <label>{"Continent"}</label>
                        <input ref={self.continent_field.clone()} class="text-input" type="text" name="continent" value={nation.core.continentName.clone()} required=true/>
                        </div>
                    }
                </form>
                {button(ctx.link().callback(|_| Msg::AddSocial), "Add Social")}

                <table class="mb-2">
                    <tr>
                        <th>{"Platform"}</th>
                        <th>{"Link"}</th>
                    </tr>
                    {for socials}
                </table>
                {button(ctx.link().callback(|_| Msg::Submit), "Submit")}
                </div>

                </>
            }
        } else {
            html! {
                <>
                {navbar!()}
                <div class="grid">
                    if is_admin || self.is_mine{
                        {button(ctx.link().callback(|_| Msg::Edit), "Edit")}
                        {button("/logout", "Logout")}
                    }
                    else if !self.logged_in{
                        {button("/discord-login", "Login to edit")}
                    }
                    else if !self.is_mine{
                        <p class="text-sm">{"This is not your nation - if this is a mistake contact the admins on discord"}</p>
                        {button("/logout", "Logout")}
                    }
                </div>

                <div class="flex flex-col place-items-center text-white">
                    {field_title("Name", &nation.core.name)}
                    {show_info(nation, flag_load, self.flag_loaded)}
                    {show_trivia(nation)}
                </div>
                </>
            }
        }
    }
}



pub type App = LoaderProcessor<Props, LoadNation, LoadNation, Nation>;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub id: i64,
}

#[async_trait(?Send)]
impl LoadProcessHandler<Props, LoadNation, LoadNation> for App {
    async fn load(props: Props) -> Result<LoadNation, String> {
        load_nation(props.id).await
    }

    fn on_load(loaded: LoadNation) -> LoadNation {
        loaded
    }
}
