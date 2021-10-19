//! This module defines a convenient `NamedBinary` struct which acts as an
//! http responder capable of returning a dynamically generated binary file
//! which is to be downloaded
//!
//! Author: X. Gillard
//! Date: September 29th, 2021

use std::io::Cursor;

use rocket::{http::Status, response::Responder, Request, Response};

/// A responder that lets the browser download a dynamically generated file
/// and give it a pre-defined name
#[derive(derive_builder::Builder)]
pub struct NamedBinary {
    /// The content-type of the binary file you return
    content_type: &'static str,
    /// The name of the file which will be downloaded
    download_name: String,
    /// The payload of the file
    payload: Vec<u8>,
}
#[rocket::async_trait]
impl<'r, 'o: 'r> Responder<'r, 'o> for NamedBinary {
    fn respond_to(self, _req: &'r Request<'_>) -> rocket::response::Result<'o> {
        Response::build()
            .streamed_body(Cursor::new(self.payload))
            .raw_header("Content-Type", self.content_type)
            .raw_header(
                "Content-Disposition",
                format!("attachment; filename=\"{}\"", self.download_name),
            )
            .ok()
    }
}
#[rocket::async_trait]
impl<'r, 'o: 'r> Responder<'r, 'o> for NamedBinaryBuilderError {
    fn respond_to(self, _req: &'r Request<'_>) -> rocket::response::Result<'o> {
        let message = format!("{:?}", self);
        Response::build()
            .sized_body(message.len(), Cursor::new(message))
            .status(Status::new(599))
            .ok()
    }
}
