use common::{current_url, LoadChanges, LoadNation, LoadNations, LoadMap, UserData, LoadResult};
use reqwasm::http::Request;

use crate::debug;

pub fn url(endpoint: &str) -> String {
    format!("{}/{}", current_url(), endpoint)
}

async fn request<T>(endpoint: &str) -> Result<LoadResult<T>, String>
where
    for<'a> T: serde::Deserialize<'a>,
{
    let payload: LoadResult<T> = Request::get(&url(endpoint))
        .send()
        .await
        .map_err(debug!())?
        .json()
        .await
        .map_err(debug!())?;

    Ok(payload)
}

macro_rules! backend {
    ($(($func_name: tt, $endpoint: expr, $type: ty)),+) => {
        $(pub async fn $func_name() -> Result<$type, String>{
            request::<$type>($endpoint).await?
        })+
    };
}

backend!(
    (load_map, "load-map", LoadMap),
    (load_nations, "nations", LoadNations),
    (user_data, "user-data", UserData),
    (nation_changes, "nation-changes", LoadChanges)
);

pub async fn load_nation(id: i64) -> Result<LoadNation, String> {
    request::<LoadNation>(&format!("nation/{}", id)).await?
}
