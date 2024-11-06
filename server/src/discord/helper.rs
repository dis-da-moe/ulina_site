use serenity::builder::{
    CreateEmbed, CreateInteractionResponseData, CreateInteractionResponseFollowup,
};
use serenity::http::Http;
use serenity::model::interactions::modal::ModalSubmitInteraction;
use serenity::model::prelude::application_command::*;
use serenity::model::prelude::*;
use serenity::utils::Colour;
use crate::error::Error;
use crate::config::CONFIG;
use crate::internal;

type Response<'a> = CreateInteractionResponseData<'a>;
type FollowUp<'a> = CreateInteractionResponseFollowup<'a>;

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

pub fn display_user(id: &str) -> String {
    format!("<@{}>", id)
}

pub fn embed() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.colour(Colour::from_rgb(15, 66, 114));
    embed
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

pub async fn is_admin(http: &Http, user: &User) -> Result<bool, Error> {
    Ok(user.id.0 == CONFIG.admin_id || user.has_role(http, CONFIG.guild_id, CONFIG.admin_role_id).await.map_err(internal!())?)
}

#[async_trait]
pub trait Helper {
    async fn message<'a, F>(&self, http: impl AsRef<Http> + Send + Sync, f: F)
    where
        F: Send,
        for<'b> F: FnOnce(&'b mut Response<'a>) -> &'b mut Response<'a>;

    async fn modal<'a, F>(&self, http: impl AsRef<Http> + Send + Sync, f: F)
    where
        F: Send,
        for<'b> F: FnOnce(&'b mut Response<'a>) -> &'b mut Response<'a>;

    async fn message_unhandled<'a, F>(
        &self,
        http: impl AsRef<Http> + Send + Sync,
        f: F,
    ) -> Result<(), serenity::Error>
    where
        F: Send,
        for<'b> F: FnOnce(&'b mut Response<'a>) -> &'b mut Response<'a>;

    async fn follow_up<'a, F>(&self, http: impl AsRef<Http> + Send + Sync, f: F)
    where
        F: Send,
        for<'b> F: FnOnce(&'b mut FollowUp<'a>) -> &'b mut FollowUp<'a>;
}

#[async_trait]
impl Helper for ApplicationCommandInteraction {
    async fn message<'a, F>(&self, http: impl AsRef<Http> + Send + Sync, f: F)
    where
        F: Send,
        for<'b> F: FnOnce(&'b mut Response<'a>) -> &'b mut Response<'a>,
    {
        if let Err(err) = self.message_unhandled(http, f).await {
            println!("Error replying to slash command: {}", err);
        };
    }

    async fn message_unhandled<'a, F>(
        &self,
        http: impl AsRef<Http> + Send + Sync,
        f: F,
    ) -> Result<(), serenity::Error>
    where
        F: Send,
        for<'b> F: FnOnce(&'b mut Response<'a>) -> &'b mut Response<'a>,
    {
        self.create_interaction_response(http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(f)
        })
        .await
    }

    async fn follow_up<'a, F>(&self, http: impl AsRef<Http> + Send + Sync, f: F)
    where
        F: Send,
        for<'b> F: FnOnce(&'b mut FollowUp<'a>) -> &'b mut FollowUp<'a>,
    {
        if let Err(err) = self.create_followup_message(http, f).await {
            println!("Error following up to slash command: {}", err);
        };
    }

    async fn modal<'a, F>(&self, http: impl AsRef<Http> + Send + Sync, f: F)
    where
        F: Send,
        for<'b> F: FnOnce(&'b mut Response<'a>) -> &'b mut Response<'a>,
    {
        if let Err(err) = self
            .create_interaction_response(http, |response| {
                response
                    .kind(InteractionResponseType::Modal)
                    .interaction_response_data(f)
            })
            .await
        {
            println!("Error replying to slash command: {}", err);
        };
    }
}

#[async_trait]
impl Helper for ModalSubmitInteraction {
    async fn message<'a, F>(&self, http: impl AsRef<Http> + Send + Sync, f: F)
    where
        F: Send,
        for<'b> F: FnOnce(&'b mut Response<'a>) -> &'b mut Response<'a>,
    {
        if let Err(err) = self.message_unhandled(http, f).await {
            println!("Error replying to modal submit: {}", err);
        };
    }

    async fn message_unhandled<'a, F>(
        &self,
        http: impl AsRef<Http> + Send + Sync,
        f: F,
    ) -> Result<(), serenity::Error>
    where
        F: Send,
        for<'b> F: FnOnce(&'b mut Response<'a>) -> &'b mut Response<'a>,
    {
        self.create_interaction_response(http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(f)
        })
        .await
    }

    async fn follow_up<'a, F>(&self, http: impl AsRef<Http> + Send + Sync, f: F)
    where
        F: Send,
        for<'b> F: FnOnce(&'b mut FollowUp<'a>) -> &'b mut FollowUp<'a>,
    {
        if let Err(err) = self.create_followup_message(http, f).await {
            println!("Error following up to modal submit: {}", err);
        };
    }

    async fn modal<'a, F>(&self, http: impl AsRef<Http> + Send + Sync, f: F)
    where
        F: Send,
        for<'b> F: FnOnce(&'b mut Response<'a>) -> &'b mut Response<'a>,
    {
        if let Err(err) = self
            .create_interaction_response(http, |response| {
                response
                    .kind(InteractionResponseType::Modal)
                    .interaction_response_data(f)
            })
            .await
        {
            println!("Error replying to slash command: {}", err);
        };
    }
}

pub type DataType = ApplicationCommandInteractionDataOptionValue;
