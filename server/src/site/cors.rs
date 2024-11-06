use common::current_url;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", current_url()));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "server, date, content-type, transfer-encoding, connection, x-frame-options, permissions-policy, x-content-type-options, access-control-allow-origin, access-control-allow-methods, access-control-allow-credentials, x-frame-options, x-content-type-options, x-xss-protection, content-encoding"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}