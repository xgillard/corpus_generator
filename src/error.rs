//! This module defines the errors I use while defining the corpus micro service
//!
//! Author: X. Gillard
//! Date: September 29th, 2021

use actix_web::ResponseError;

/// This enum covers all the cases in which the corpus micro service can fail
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// When an io error is raised
    #[error("io error {0}")]
    Io(#[from] std::io::Error),
    /// When it fails to properly generate a named binary response
    /// (should never occur though)
    #[error("error building the binary response {0}")]
    NamedBinary(#[from] crate::named_binary::NamedBinaryBuilderError),
    /// This error occurs when there are more than one private key in the 
    /// file which is passed as server PK.
    #[error("There should be one and only one private key: found {0}")]
    TooMayPrivateKeys(usize),
    /// This error occurs when something goes wrong with rustls
    #[error("error with TLS: {0}")]
    Tls(#[from] rustls::TLSError),
}

/// This is a convenient alias I'll use to tell that any result I return
/// may either be some value or one of the above errors
pub type Result<T> = std::result::Result<T, Error>;

impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}
