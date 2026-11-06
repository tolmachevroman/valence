//! The error every data source returns.

use std::fmt;

use crate::http::HttpError;

#[derive(Debug)]
pub enum Error {
    Http(HttpError),
    /// The source answered, but not in the shape we expect, like `PubChem CID list`.
    Format(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http(e) => e.fmt(f),
            Self::Format(what) => write!(f, "unexpected {what}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<HttpError> for Error {
    fn from(e: HttpError) -> Self {
        Self::Http(e)
    }
}
