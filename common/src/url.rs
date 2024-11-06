pub const LOCAL_IP: &str = "127.0.0.1";
pub const PORT: u16 = 8000;
pub const URL: &str = "https://www.ulinaworld.com";
use lazy_static::lazy_static;

lazy_static! {
    pub static ref LOCAL_URL: String = format!("http://{}:{}", LOCAL_IP, PORT);
}

pub fn current_url() -> &'static str {
    if cfg!(debug_assertions) {
        LOCAL_URL.as_str()
    } else {
        URL
    }
}
