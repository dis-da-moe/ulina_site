use serenity::builder::CreateInteractionResponseData;
use serenity::http::Http;
use serenity::model::prelude::application_command::*;
use serenity::model::prelude::*;

#[async_trait]
pub trait Helper {
    async fn message<'a, F>(&self, http: impl AsRef<Http> + Send + Sync, f: F)
    where
        F: Send,
        for<'b> F: FnOnce(
            &'b mut CreateInteractionResponseData<'a>,
        ) -> &'b mut CreateInteractionResponseData<'a>;
}

#[async_trait]
impl Helper for ApplicationCommandInteraction {
    async fn message<'a, F>(&self, http: impl AsRef<Http> + Send + Sync, f: F)
    where
        F: Send,
        for<'b> F: FnOnce(
            &'b mut CreateInteractionResponseData<'a>,
        ) -> &'b mut CreateInteractionResponseData<'a>,
    {
        if let Err(err) = self
            .create_interaction_response(http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(f)
            })
            .await
        {
            println!("Error replying to slash command: {}", err);
        };
    }
}

pub type DataType = ApplicationCommandInteractionDataOptionValue;

#[macro_export]
macro_rules! get_options {
    ($options: expr, $($name: expr, $value_type: ident),+) => {
        (
            $(match crate::discord::helper::option(&$options, $name) {
                Some(crate::discord::helper::DataType::$value_type(x, ..)) => Ok(x),
                None => Err(crate::error::Error::InternalError(format!("No option by the name of \"{}\"", $name))),
                _ => Err(crate::error::Error::InternalError(format!("Option by name {} has wrong type", $name)))
            }),+
        )
    };
}

pub fn option<'a>(
    options: &'a Vec<ApplicationCommandInteractionDataOption>,
    name: &str,
) -> Option<&'a DataType> {
    options
        .iter()
        .find(|option| option.name == name)?
        .resolved
        .as_ref()
}

pub fn is_admin(user: &User) -> bool {
    user.id.0.to_string() == "368673056899596290"
}
