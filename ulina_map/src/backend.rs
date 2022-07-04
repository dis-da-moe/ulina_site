use common::{LoadMap, LOCAL_URL};
use reqwasm::http::Request;

macro_rules! debug {
    () => {
        |err| format!("{:?}", err)
    };
}

pub fn url(endpoint: &str) -> String {
    #[cfg(debug_assertions)]
    let url = LOCAL_URL.as_str();

    #[cfg(not(debug_assertions))]
    let url = shared::URL;

    format!("{}/{}", url, endpoint)
}

pub async fn request<T>(endpoint: &str) -> Result<T, String>
where for<'a> T: serde::Deserialize<'a>
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