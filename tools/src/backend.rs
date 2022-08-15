use common::{current_url, LoadMap, LoadNation, LoadNations};
use reqwasm::http::Request;

use crate::debug;

pub fn url(endpoint: &str) -> String {
    format!("{}/{}", current_url(), endpoint)
}

pub async fn request<T>(endpoint: &str) -> Result<T, String>
where
    for<'a> T: serde::Deserialize<'a>,
{
    let payload: T = Request::get(&url(endpoint))
        .send()
        .await
        .map_err(debug!())?
        .json()
        .await
        .map_err(debug!())?;

    Ok(payload)
}

pub async fn load_map() -> Result<LoadMap, String> {
    request("load-map").await
}

pub async fn load_nations() -> Result<LoadNations, String> {
    request("nations").await
}

pub async fn load_nation(id: i64) -> Result<LoadNation, String> {
    request(&format!("nation/{}", id)).await
}
