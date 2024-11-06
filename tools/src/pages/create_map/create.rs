use std::mem::take;

use async_trait::async_trait;
use common::{LoadNations, NationContinentId};
use gloo::file::File;
use gloo::file::callbacks::FileReader;
use web_sys::{HtmlInputElement, HtmlFormElement};
use yew::prelude::*;

use super::nation::CreateNation;

use crate::{loader::{Loader, LoadHandler, LoadProps}, backend, util::{by_id, get_vec, XMLNS, viewbox::Viewbox}, components::button};

pub type App = Loader<LoadNations, Create>;

type CreateProps = LoadProps<LoadNations>;

#[async_trait(?Send)]
impl LoadHandler<LoadNations> for App {
    async fn load() -> Result<LoadNations, String> {
        backend::load_nations().await
    }
    fn on_load(mut loaded: LoadNations) -> LoadNations {
        loaded.data.retain(|nation| !nation.removed);
        loaded
    }
}

pub struct Create{
    reader: Option<FileReader>,
    svg: Option<(Vec<NamedNation>, Viewbox, String)>,
    current_nation: usize,
    form: NodeRef,
    svg_input: NodeRef
}

pub struct NationRegionIndex(usize);
pub enum Msg{
    Drop(File),
    Drag,
    Loaded(String),
    Clicked(NationRegionIndex),
    Submit
}

struct NamedNation{
    inner: String,
    id: Option<String>,
    html: Html
}

impl Create{
    fn current<'a, 'b>(&'a self, ctx: &'b Context<Create>) -> Option<&'b NationContinentId>{
        ctx.props().loaded.data.get(self.current_nation)
    }

    fn current_nation_name(&self, ctx: &Context<Create>) -> Option<String>{
        self.current(ctx).map(|nation| nation.name.clone())
    }

    fn next_nation_id(&self, ctx: &Context<Create>) -> Option<String>{
        let current = self.current(ctx).clone().map(|nation| {
            nation.nationId.to_string()
        });
        current
    }
}

impl Component for Create{
    type Message = Msg;

    type Properties = CreateProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self{
            reader: None,
            svg: None,
            current_nation: 0,
            svg_input: NodeRef::default(),
            form: NodeRef::default()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg{
            Msg::Drop(file) => {
                let link = ctx.link().clone();

                
                self.reader = Some(gloo::file::callbacks::read_as_text(&file, move |res|{
                    link.send_message(Msg::Loaded(res.unwrap()));
                }));
                false
            },
            Msg::Loaded(file) => {
                self.reader = None;
                let div = gloo_utils::document().create_element("div").unwrap();
                div.set_inner_html(&file);
                
                let nation_elements = by_id(&div, "NATIONS".to_string()).unwrap();

                let mut claims = None;
                
                let children: Vec<NamedNation> = get_vec(&nation_elements.children()).into_iter()
                    .filter_map(|element|{
                    
                    if element.id().as_str() == "claims"{
                        claims = Some(element);
                        return None;
                    }
                        
                    match element.tag_name().as_str(){
                        "path" => {
                            let group = gloo_utils::document().create_element_ns(Some(XMLNS),"g").unwrap();
                            group.append_child(&element.clone().into()).unwrap();
                            Some(group)
                        },
                        "g" => Some(element),
                        _ => None
                    }
                    
                })
                .enumerate()
                .map(|(index, element)|{
                    let callback = ctx.link().callback(move|_: MouseEvent| Msg::Clicked(NationRegionIndex(index)));
                    
                    let id = if element.id().is_empty(){ None } else{ Some(element.id()) };

                    NamedNation { 
                        inner: element.inner_html(),
                        id: id.clone(),
                        html: html!{<CreateNation inner={element.inner_html()} id={id} onclick={Some(callback)} assigned={false}/>} 
                    }
                }).collect();
                let viewbox = div.first_element_child().unwrap().get_attribute("viewBox").unwrap().parse().unwrap();

                self.svg = Some((children, viewbox, claims.unwrap().inner_html()));

                true
            },
            Msg::Clicked(NationRegionIndex(index)) => {
                let nation_id = self.next_nation_id(ctx);

                let nation = self.svg.as_mut().unwrap().0.get_mut(index).unwrap();
                

                nation.id = nation_id.clone();
                
                self.current_nation += 1;
                
                nation.html = html!{
                    <CreateNation inner={nation.inner.clone()} id={nation.id.clone()} onclick={None} assigned={true}/>
                };
                true
            },
            Msg::Submit => {
                let (mut nations, viewbox, claims) = take(self.svg.as_mut().unwrap());
                let svg = gloo_utils::document().create_element_ns(Some(XMLNS), "svg").unwrap();
                svg.set_attribute("viewBox", &viewbox.to_string()).unwrap();
                svg.set_attribute("xmlns", XMLNS).unwrap();
                svg.set_id("NATIONS");
                nations.push(NamedNation { 
                    inner: claims, 
                    id: Some("claims".to_string()), 
                    html: html!{} 
                });

                for nation in nations{
                    let group = gloo_utils::document().create_element_ns(Some(XMLNS), "g").unwrap();
                    group.set_inner_html(&nation.inner);
                    group.set_id(&nation.id.unwrap());
                    svg.append_child(&group.into()).unwrap();
                }
                self.svg_input.cast::<HtmlInputElement>().unwrap().set_value(&svg.outer_html());
                self.form.cast::<HtmlFormElement>().unwrap().submit().unwrap();
                true
            }
            _ => {false}
        }

        
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ondrop = ctx.link().callback(|e: DragEvent| {
            e.stop_propagation();
            e.prevent_default();

            Msg::Drop(e.data_transfer().unwrap().files().unwrap().get(0).unwrap().into())
        });
        let ondragover = ctx.link().callback(|e: DragEvent|{
            e.stop_propagation();
            e.prevent_default();
            e.data_transfer().unwrap().set_drop_effect("copy");
            Msg::Drag
        });
        
        html!{
            <>
            if let Some((children, viewbox, _)) = &self.svg{
                if children.len() != ctx.props().loaded.data.len(){
                    <h1>{format!("Not the same amount of nations, {} vs {}", children.len(), ctx.props().loaded.data.len())}</h1>
                }
                else if let Some(name) = self.current_nation_name(ctx){

                    <h1 class="sticky top-0">{name}</h1>
                    <div class="h-screen">
                        <svg viewBox={viewbox.to_string()} xmlns={XMLNS}>
                            {for children.iter().map(|child| child.html.clone())}
                        </svg>
                    </div>

                }
                else{
                    <form action="/create-map" method="POST" enctype="multipart/form-data" ref={self.form.clone()}>
                        <input type="text" name="svg" ref={self.svg_input.clone()} hidden={true}/>
                    </form>
                    {button(ctx.link().callback(|_| Msg::Submit), "submit")}
                }
            }
            else{
                <div class="h-screen" {ondrop} {ondragover}>
                
                </div>
            }
            
            </>
        }
    }
}