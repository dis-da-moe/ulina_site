use rocket::{
    http::{Cookie, CookieJar, SameSite, Status},
    request::FromRequest,
    request::Outcome,
    Request,
};
use serde::Deserialize;
use sqlx::query;

use crate::database::db;

#[derive(Debug)]
pub struct UserId(pub i64);

#[derive(Debug)]
pub enum UserError {
    InvalidUser,
    NotAdmin,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserId {
    type Error = UserError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(if let Some(cookie) = get_user(req.cookies()) {
            if let Some(id) = cookie.value().parse::<i64>().ok() {
                if valid_id(id).await {
                    UserId(id)
                } else {
                    reset_user(req.cookies()).await
                }
            } else {
                reset_user(req.cookies()).await
            }
        } else {
            add_user(req.cookies()).await
        })
    }
}

pub struct AdminUser(pub i64);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = UserError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user_outcome = req.guard::<UserId>().await;

        let id = match user_outcome {
            Outcome::Success(id) => id,
            Outcome::Failure(e) => return Outcome::Failure(e),
            Outcome::Forward(_) => return Outcome::Forward(()),
        };

        if query!("SELECT isAdmin FROM User WHERE userId = ?", id.0)
            .fetch_one(db())
            .await
            .unwrap()
            .isAdmin
        {
            Outcome::Success(AdminUser(id.0))
        } else {
            Outcome::Forward(())
        }
    }
}

fn get_user<'r>(cookies: &CookieJar<'r>) -> Option<Cookie<'static>> {
    cookies.get_private("userId")
}
async fn valid_id(id: i64) -> bool {
    query!("SELECT userId FROM User WHERE userId = ?", id)
        .fetch_one(db())
        .await
        .is_ok()
}

async fn add_user<'r>(cookies: &CookieJar<'r>) -> UserId {
    let id = query!("INSERT INTO User (isAdmin) VALUES (false) RETURNING userId")
        .fetch_one(db())
        .await
        .unwrap()
        .userId;

    let mut cookie = Cookie::new("userId", id.to_string());
    cookie.set_same_site(SameSite::Lax);
    cookie.set_secure(Some(true));
    cookies.add_private(cookie);

    UserId(id)
}

async fn reset_user<'r>(cookies: &CookieJar<'r>) -> UserId {
    cookies.remove_private(Cookie::named("userId"));
    add_user(cookies).await
}

#[derive(Deserialize, FromForm)]
pub struct Login {
    pub password: String,
}
