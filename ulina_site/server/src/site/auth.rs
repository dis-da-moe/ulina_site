use crate::config::CONFIG;
use crate::database::db;
use crate::error::Error;
use crate::site::user_data::UserId;
use crate::util::StandardClient;
use common::current_url;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::url::Url;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use rocket::form::Form;
use rocket::response::content::RawHtml;
use rocket::response::Redirect;
use rocket_governor::{Quota, RocketGovernable, RocketGovernor};
use serde::Deserialize;
use sqlx::query;
use sycamore::view::View;
use sycamore::{view, SsrNode};

use super::rendering::Render;
use super::user_data::{AdminUser, Login};

lazy_static! {
    static ref REDIRECT_URL: String = format!("{}/oauth-redirect", current_url());
    pub static ref OAUTH_CLIENT: StandardClient = BasicClient::new(
        ClientId::new(CONFIG.client_id.to_string()),
        Some(ClientSecret::new(CONFIG.client_secret.clone())),
        AuthUrl::new("https://discord.com/api/oauth2/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://discord.com/api/oauth2/token".to_string()).unwrap())
    )
    .set_redirect_uri(RedirectUrl::new(REDIRECT_URL.clone()).unwrap());
}

fn oauth_url() -> (Url, CsrfToken) {
    OAUTH_CLIENT
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .url()
}

#[get("/discord-login")]
pub async fn discord_login(user_id: UserId) -> Result<Redirect, Error> {
    let (url, token) = oauth_url();
    let token = token.secret();

    query!(
        "UPDATE User SET pendingAuth = ? WHERE userId = ?",
        token,
        user_id.0
    )
    .execute(db())
    .await?;

    Ok(Redirect::temporary(url.to_string()))
}

#[derive(Deserialize)]
struct DiscordResponse {
    id: String,
    username: String,
}

#[get("/oauth-redirect?<code>&<state>")]
pub async fn oauth_redirect(
    code: String,
    state: String,
    user: UserId,
) -> Result<RawHtml<String>, Error> {
    let message = |message: String| view! {p{(message)}}.render();
    let stored_state = query!("SELECT pendingAuth FROM User WHERE userId = ?", user.0)
        .fetch_one(db())
        .await?
        .pendingAuth;

    if let Some(stored_state) = stored_state {
        query! {"UPDATE User SET pendingAuth = NULL WHERE userId = ?", user.0}
            .execute(db())
            .await?;

        if stored_state != state {
            return Ok(message("Invalid state".to_string()));
        }
    } else {
        return Ok(message("No request made yet".to_string()));
    }

    let token_result = OAUTH_CLIENT
        .exchange_code(AuthorizationCode::new(code.to_string()))
        .request_async(async_http_client)
        .await;

    let auth_code = match token_result {
        Ok(token) => token.access_token().secret().clone(),
        Err(e) => {
            return Ok(message(format!(
                "An error occured while exchanging codes: {:?}",
                e
            )));
        }
    };
    let client = reqwest::Client::new();
    let response = client
        .get("https://discord.com/api/users/@me")
        .bearer_auth(auth_code)
        .send()
        .await?
        .json::<DiscordResponse>()
        .await;

    match response {
        Ok(response) => {
            query!(
                "UPDATE User SET discord = ? WHERE userId = ?",
                response.id,
                user.0
            )
            .execute(db())
            .await?;
            let nation = query!(
                "SELECT nationId FROM Nation WHERE ownerDiscord = ?",
                response.id
            )
            .fetch_one(db())
            .await
            .map(|nation| format!("/tools/nation/{}", nation.nationId));

            let button: View<SsrNode> = if let Ok(link) = nation {
                view! {
                    a(href=link){"View nation"}
                }
            } else {
                view! {
                    p{"You do not have a nation."}
                }
            };

            Ok(view! {
                p{(format!("successfully signed in as {}", response.username))}

                (button)
            }
            .render())
        }
        Err(e) => Ok(message(e.to_string())),
    }
}

#[get("/admin", rank = 1)]
pub async fn admin(_user: AdminUser) -> RawHtml<String> {
    view! {
        div{"Logged in as admin"}
        a(href="/tools"){"Click to view tools"}
    }
    .render()
}

#[get("/admin?<error>", rank = 2)]
pub async fn admin_login(error: Option<String>) -> RawHtml<String> {
    view! {
        form(action = "/login-result", method="POST"){
            input(name="password", type="password", placeholder="Password", class="text-input", required=true)
            input(type="submit", name="submit", value="submit")
        }
        (if let Some(message) = error.clone(){
            view!{
                p{(message)}
            }
        }else{view!{}})
    }
    .render()
}

#[post("/login-result", data = "<login>")]
pub async fn login_result(
    login: Form<Login>,
    user: UserId,
    admin: Option<AdminUser>,
    limitguard: Option<RocketGovernor<'_, LimitLogin>>,
) -> Result<Redirect, Error> {
    let success = Redirect::to(uri!(admin));

    if admin.is_some() {
        return Ok(success);
    }

    let error = |message: &str| Redirect::to(uri!(admin_login(Some(message.to_string()))));

    if limitguard.is_none() {
        return Ok(error("too many tries"));
    }

    if login.password == CONFIG.admin.as_str() {
        query!("UPDATE User SET isAdmin = true WHERE userId = ?", user.0)
            .execute(db())
            .await?;
        Ok(success)
    } else {
        Ok(error("incorrect password"))
    }
}

#[get("/logout")]
pub async fn logout(user: UserId) -> Result<RawHtml<String>, Error> {
    query!(
        "UPDATE User SET isAdmin = false, discord = NULL WHERE userId = ?",
        user.0
    )
    .execute(db())
    .await?;

    Ok(view!(
        p{"logged out successfully"}
    )
    .render())
}

pub struct LimitLogin;

impl<'r> RocketGovernable<'r> for LimitLogin {
    fn quota(_method: rocket_governor::Method, _route_name: &str) -> rocket_governor::Quota {
        Quota::per_hour(Self::nonzero(5))
    }
}
