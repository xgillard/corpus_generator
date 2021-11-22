//! This module defines a convenient `NamedBinary` struct which acts as an
//! http responder capable of returning a dynamically generated binary file
//! which is to be downloaded
//!
//! Author: X. Gillard
//! Date: September 29th, 2021

use std::future::{Ready, ready};
use actix_web::{HttpRequest, HttpResponse, Responder, http, web::Bytes};

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

impl Responder for NamedBinary {
    type Error  = actix_web::Error;
    type Future = Ready<Result<HttpResponse, Self::Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let response = HttpResponse::Ok()
            .set_header(http::header::CONTENT_TYPE, self.content_type)
            .set_header(http::header::CONTENT_DISPOSITION, format!("attachment; filename=\"{}\"", self.download_name))
            .body(Bytes::from(self.payload))
            ;
        ready(Ok(response))
    }
}
