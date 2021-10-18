//! This module deines a handy "fairing" which allows cross origin resource 
//! sharing. It adds some headers to requests and responses to allow 
//! responses from a different domain than the one originating the request.
//!
//! Note: It would probably have been cleaner to just reuse the `rocket_cors`
//! crate, but it requires a nightly toolchain which is not intalled on my 
//! vm.


use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

/// This is the object which can be attached to rocket to enable CORS.
#[derive(Debug, Clone, Copy)]
pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response
        }
    }
    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
