use std::collections::HashMap;

use crate::back;
use crate::components::Flag;
use async_trait::async_trait;
use common::{AddSocial, LoadNation, CONTINENTS};
use web_sys::{HtmlFormElement, HtmlInputElement};
use yew::prelude::*;

use crate::util::{input_checkbox, input_text, BUTTON_CLASS};
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

pub enum Msg {
    FlagLoaded,
    OptionalField(&'static str, String),
    Submit,
    AddSocial,
    RemoveSocial(usize),
    EditSocial(usize, String, fn(&mut AddSocial) -> &mut String),
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
            Msg::RemoveSocial(index) => {
                self.socials.remove(index);
            }
            Msg::EditSocial(index, value, editor) => {
                *editor(self.socials.get_mut(index).unwrap()) = value
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
                        <div>
                        <label>{name.to_string()}</label>
                        <textarea type="text" name={name.to_string()} value={value.clone().unwrap_or("".to_string())} {oninput}/>
                        </div>
                    }
                });

            let edit_social = |index, editor| {
                ctx.link()
                    .callback(move |e| Msg::EditSocial(index, input_text(e), editor))
            };

            let socials = self.socials.iter().enumerate().map(|(index, social)|{
                let edit = |editor| edit_social(index, editor);

                html!{
                    <tr>
                        <td><input value={social.platform.clone()} oninput={edit(|social| &mut social.platform)}/></td>
                        <td><input value={social.link.clone()} oninput={edit(|social| &mut social.link)}/></td>
                        <td><a class={BUTTON_CLASS} onclick={ctx.link().callback(move |_| Msg::RemoveSocial(index))}>{"Delete"}</a></td>
                    </tr>
                }
            });

            html! {
                <>
                {back!()}

                if let Some(message) = self.message.clone(){
                    <p>{message}</p>
                }

                <form ref={self.form.clone()} action="/edit-nation" method="POST" enctype="multipart/form-data">
                    {input_field("id", nation.core.nationId, true, true)}
                    <input type="text" name="socials" value="" hidden=true ref={self.socials_field.clone()}/>

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
                        <input type="checkbox" required=false name="removed" checked={self.removed} oninput={ctx.link().callback(|e| Msg::Removed(input_checkbox(e)))}/>
                        </div>

                        {input_field("discord", &nation.core.ownerDiscord, false, true)}
                        <div>
                        <label>{"Continent"}</label>
                        <input ref={self.continent_field.clone()} type="text" name="continent" value={nation.core.continentName.clone()} required=true/>
                        </div>
                    }
                </form>

                <a class={BUTTON_CLASS} onclick={ctx.link().callback(|_| Msg::AddSocial)}>{"Add Social"}</a>

                <table>
                    <tr>
                        <th>{"Platform"}</th>
                        <th>{"Link"}</th>
                    </tr>
                    {for socials}
                </table>

                <a onclick={ctx.link().callback(|_| Msg::Submit)} class={BUTTON_CLASS}>{"Submit"}</a>
                </>
            }
        } else {
            html! {
                <>
                {back!()}
                if is_admin || self.is_mine{
                    <a class={BUTTON_CLASS} onclick={ctx.link().callback(|_| Msg::Edit)}>{"Edit"}</a>
                }
                else if !self.logged_in{
                    <a class={BUTTON_CLASS} href="/discord-login">{"Click to login with discord to edit"}</a>
                }
                else if !self.is_mine{
                    <p>{"This is not your nation - if this is a mistake contact the admins on discord"}</p>
                }


                <div class="flex flex-col place-items-center">
                    {field_title("Name", &nation.core.name)}
                    {show_info(nation, flag_load, self.flag_loaded)}
                    {show_trivia(nation)}
                </div>
                </>
            }
        }
    }
}

fn input_field<T: ToString, Y: ToString>(name: Y, value: T, hidden: bool, required: bool) -> Html {
    html! {
        <div>
        if !hidden{
            <label>{name.to_string()}</label>
        }
        <input type="text" name={name.to_string()} value={value.to_string()} hidden={hidden} required={required}/>
        </div>
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
