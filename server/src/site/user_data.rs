use rocket::{
    http::{Cookie, CookieJar, SameSite, Status},
    request::FromRequest,
    request::Outcome,
    Request, outcome::IntoOutcome,
};
use serde::Deserialize;
use sqlx::query;

use crate::{database::db, error::Error};

#[derive(Debug)]
pub struct UserId(pub i64);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserId {
    type Error = Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let result = if let Some(cookie) = get_user(req.cookies()) {
            if let Some(id) = cookie.value().parse::<i64>().ok() {
                if valid_id(id).await {
                    Ok(UserId(id))
                } else {
                    reset_user(req.cookies()).await
                }
            } else {
                reset_user(req.cookies()).await
            }
        } else {
            add_user(req.cookies()).await
        };

        result.into_outcome(Status::InternalServerError)
    }
}

pub struct AdminUser(pub i64);

impl From<AdminUser> for UserId{
    fn from(user: AdminUser) -> Self {
        UserId(user.0)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user_outcome = req.guard::<UserId>().await;

        let id = match user_outcome {
            Outcome::Success(id) => id,
            Outcome::Failure(e) => return Outcome::Failure(e),
            Outcome::Forward(_) => return Outcome::Forward(()),
        };

        match query!("SELECT isAdmin FROM User WHERE userId = ?", id.0)
        .fetch_one(db())
        .await{
            Err(e) => {
                Outcome::Failure((Status::InternalServerError, e.into()))
            }
            Ok(result) if result.isAdmin => {
                Outcome::Success(AdminUser(id.0))
            }
            _ => {
                Outcome::Forward(())
            }
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

async fn add_user<'r>(cookies: &CookieJar<'r>) -> Result<UserId, Error> {
    //TODO: add `lastVisited` field to `User` table and schedule task to remove users who have not signed in for more than a week  
    let id = query!("INSERT INTO User (isAdmin) VALUES (false) RETURNING userId")
        .fetch_one(db())
        .await?
        .userId;

    let mut cookie = Cookie::new("userId", id.to_string());
    cookie.set_same_site(SameSite::None);
    cookie.set_secure(Some(true));
    cookies.add_private(cookie);

    Ok(UserId(id))
}

async fn reset_user<'r>(cookies: &CookieJar<'r>) -> Result<UserId, Error> {
    cookies.remove_private(Cookie::named("userId"));
    add_user(cookies).await
}

#[derive(Deserialize, FromForm)]
pub struct Login {
    pub password: String,
}
