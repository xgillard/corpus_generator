//! This module defines the errors I use while defining the corpus micro service
//!
//! Author: X. Gillard
//! Date: September 29th, 2021

/// This enum covers all the cases in which the corpus micro service can fail
#[derive(Debug, thiserror::Error, rocket::response::Responder)]
pub enum Error {
    /// When an io error is raised
    #[error("io error {0}")]
    Io(#[from] std::io::Error),
    /// When it fails to properly generate a named binary response
    /// (should never occur though)
    #[error("error building the binary response {0}")]
    NamedBinary(#[from] crate::named_binary::NamedBinaryBuilderError),
}

/// This is a convenient alias I'll use to tell that any result I return
/// may either be some value or one of the above errors
pub type Result<T> = std::result::Result<T, Error>;
