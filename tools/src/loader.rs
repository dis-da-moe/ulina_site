use std::marker::PhantomData;

use crate::components::Error;
use crate::components::Loading;
use async_trait::async_trait;
use yew::prelude::*;

pub struct LoaderProcessor<Props, Data, ProcessedData, Comp> {
    props_data: Option<Result<ProcessedData, String>>,
    data: PhantomData<Data>,
    child: PhantomData<Comp>,
    props: PhantomData<Props>,
}

pub enum Msg<Data> {
    Loaded(Result<Data, String>),
}

#[derive(Debug, Properties, PartialEq)]
pub struct LoadProps<ProcessedData: PartialEq> {
    pub loaded: ProcessedData,
}

#[async_trait(?Send)]
pub trait LoadProcessHandler<Props, Data, ProcessedData> {
    async fn load(props: Props) -> Result<Data, String>;

    fn on_load(loaded: Data) -> ProcessedData;
}

#[async_trait(?Send)]
pub trait LoadHandler<Data> {
    async fn load() -> Result<Data, String>;

    fn on_load(loaded: Data) -> Data {
        loaded
    }
}

#[async_trait(?Send)]
impl<Data, Handler: LoadHandler<Data>> LoadProcessHandler<(), Data, Data> for Handler {
    async fn load(_: ()) -> Result<Data, String> {
        Handler::load().await
    }

    fn on_load(loaded: Data) -> Data {
        Handler::on_load(loaded)
    }
}

pub type Loader<Data, Comp> = LoaderProcessor<(), Data, Data, Comp>;

impl<Props, Data, ProcessedData, Comp> Component
    for LoaderProcessor<Props, Data, ProcessedData, Comp>
where
    Props: 'static + Properties + Clone,
    Data: 'static,
    ProcessedData: 'static + PartialEq + Clone,
    Comp: 'static + Component<Properties = LoadProps<ProcessedData>>,
    LoaderProcessor<Props, Data, ProcessedData, Comp>:
        LoadProcessHandler<Props, Data, ProcessedData>,
{
    type Message = Msg<Data>;

    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props().clone();
        ctx.link().send_future(async move {
            Msg::<Data>::Loaded(
                LoaderProcessor::<Props, Data, ProcessedData, Comp>::load(props).await,
            )
        });
        LoaderProcessor {
            props_data: None,
            props: PhantomData,
            data: PhantomData,
            child: PhantomData,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let data = match msg {
            Msg::Loaded(data) => data,
        };
        self.props_data = Some(
            data.map(|data| LoaderProcessor::<Props, Data, ProcessedData, Comp>::on_load(data)),
        );
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match &self.props_data {
            None => html! {<Loading/>},
            Some(Err(error)) => html! {<Error error_message={error.clone()}/>},
            Some(Ok(value)) => html! {
                <Comp loaded={value.clone()}/>
            },
        }
    }
}
