use common::{current_url, LoadMap};
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

pub async fn start_load() -> Result<LoadMap, String> {
    request("load-map").await
}
