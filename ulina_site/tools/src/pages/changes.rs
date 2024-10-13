use crate::{
    backend,
    loader::{LoadHandler, LoadProps, Loader},
    navbar,
};
use async_trait::async_trait;
use common::DATE_FORMAT;
use common::{ChangeType, LoadChanges};
use yew::prelude::*;
pub type App = Loader<LoadChanges, Changes>;

type ChangesProps = LoadProps<LoadChanges>;

#[async_trait(?Send)]
impl LoadHandler<LoadChanges> for App {
    async fn load() -> Result<LoadChanges, String> {
        backend::nation_changes().await
    }

    fn on_load(mut loaded: LoadChanges) -> LoadChanges {
        loaded.data.sort_by(|a, b| a.date.cmp(&b.date));
        loaded
    }
}

pub struct Changes;

macro_rules! row_values {
    ($($value: expr),+ $(=> $extra_class: expr)?) => {
    {   let _extra: Option<&str> = None;
        $(let _extra = $extra_class;)?

        html!{
            <>
            $(<td class={classes!("border-x", "border-gray-600", "text-center", _extra)}>
                {$value}
            </td>)+
            </>
        }
    }
    };
}

fn default_convert(val: &Option<String>) -> Html {
    row_values!(val.clone().unwrap_or_else(|| "NULL".to_string()))
}

fn flag_convert(val: &Option<String>) -> Html {
    match val {
        None => default_convert(val),
        Some(value) => row_values!(html! {
            <img src={value.clone()} class="w-[20%] m-auto"/>
        }),
    }
}

impl Component for Changes {
    type Message = ();

    type Properties = ChangesProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Changes {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let changes = ctx.props().loaded.data.iter().map(|change| {
            let convert = match change.change_type {
                ChangeType::Flag => flag_convert,
                _ => default_convert,
            };

            html! {
                <tr class="border-y border-gray-600">
                {row_values!(change.nation_name.clone(), change.change_type.to_string())}
                {convert(&change.old_value)}
                {convert(&change.new_value)}
                {row_values!(change.date.format(DATE_FORMAT).to_string(), change.admin.to_string())}
                </tr>
            }
        });

        html! {
            <>
                {navbar!()}
                <h1>{"Nation Changes"}</h1>
                <table>
                    <tr>
                        {row_values!(
                            "Nation",
                            "Type",
                            "Old Value",
                            "New Value",
                            "Time",
                            "Done by admin"
                            => "bold"
                        )}
                    </tr>
                    {for changes}
                </table>
            </>
        }
    }
}
