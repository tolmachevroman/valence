//! The error every data source and the cache return.

use std::fmt;

use crate::http::HttpError;

#[derive(Debug)]
pub enum Error {
    Http(HttpError),
    /// The source answered, but not in the shape we expect, like `PubChem CID list`.
    Format(&'static str),
    /// The local cache failed.
    Cache(rusqlite::Error),
}

impl Error {
    /// No answer from the network: offline, or the connection failed.
    pub fn is_offline(&self) -> bool {
        matches!(self, Self::Http(HttpError::Network(_)))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http(e) => e.fmt(f),
            Self::Format(what) => write!(f, "unexpected {what}"),
            Self::Cache(e) => write!(f, "cache: {e}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<rusqlite::Error> for Error {
    fn from(e: rusqlite::Error) -> Self {
        Self::Cache(e)
    }
}

impl From<HttpError> for Error {
    fn from(e: HttpError) -> Self {
        Self::Http(e)
    }
}
