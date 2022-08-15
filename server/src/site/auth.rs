use crate::config::CONFIG;
use crate::database::db;
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
use sycamore::view;

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
pub async fn discord_login(user_id: UserId) -> RawHtml<String> {
    let (url, token) = oauth_url();
    let token = token.secret();

    query!(
        "UPDATE User SET pendingAuth = ? WHERE userId = ?",
        token,
        user_id.0
    )
    .execute(db())
    .await
    .unwrap();

    view! {
        a(href={url.as_str()}) {"Click here to log in"}
    }
    .render()
}

#[derive(Deserialize)]
struct DiscordResponse {
    id: String,
    username: String,
}

#[get("/oauth-redirect?<code>&<state>")]
pub async fn oauth_redirect(code: String, state: String, user: UserId) -> RawHtml<String> {
    let message = |message: String| view! {p{(message)}}.render();
    let stored_state = query!("SELECT pendingAuth FROM User WHERE userId = ?", user.0)
        .fetch_one(db())
        .await
        .unwrap()
        .pendingAuth;

    if let Some(stored_state) = stored_state {
        query! {"UPDATE User SET pendingAuth = NULL WHERE userId = ?", user.0}
            .execute(db())
            .await
            .unwrap();

        if stored_state != state {
            return message("Invalid state".to_string());
        }
    } else {
        return message("No request made yet".to_string());
    }

    let token_result = OAUTH_CLIENT
        .exchange_code(AuthorizationCode::new(code.to_string()))
        .request_async(async_http_client)
        .await;

    let auth_code = match token_result {
        Ok(token) => token.access_token().secret().clone(),
        Err(e) => {
            return message(format!("An error occured while exchanging codes: {:?}", e));
        }
    };
    let client = reqwest::Client::new();
    let response = client
        .get("https://discord.com/api/users/@me")
        .bearer_auth(auth_code)
        .send()
        .await
        .unwrap()
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
            .await
            .unwrap();
            view!{
                p{(format!("successfully signed in as {}", response.username))}
                a(href="/tools/nations"){"View Nations"}
            }.render()
        }
        Err(e) => message(e.to_string()),
    }
}

#[get("/admin", rank = 1)]
pub async fn admin(_user: AdminUser) -> RawHtml<String> {
    view! {div{"This is the admin page"}}.render()
}

#[get("/admin?<error>", rank = 2)]
pub async fn admin_login(error: Option<String>) -> RawHtml<String> {
    view! {
        form(action = "/login-result", method="POST"){
            input(name="password", type="password", placeholder="Password", required=true)
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
) -> Redirect {
    let success = Redirect::to(uri!(admin));

    if admin.is_some() {
        return success;
    }

    let error = |message: &str| Redirect::to(uri!(admin_login(Some(message.to_string()))));

    if limitguard.is_none() {
        return error("too many tries");
    }

    if login.password == CONFIG.admin.as_str() {
        query!("UPDATE User SET isAdmin = true WHERE userId = ?", user.0)
            .execute(db())
            .await
            .unwrap();
        success
    } else {
        error("incorrect password")
    }
}
pub struct LimitLogin;

impl<'r> RocketGovernable<'r> for LimitLogin {
    fn quota(_method: rocket_governor::Method, _route_name: &str) -> rocket_governor::Quota {
        Quota::per_hour(Self::nonzero(5))
    }
}
