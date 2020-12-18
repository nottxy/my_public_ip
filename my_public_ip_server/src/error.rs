use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("read config file error: {0}")]
    ReadConfigError(io::Error),
    #[error("parse config file error: {0}")]
    ParseConfigError(toml::de::Error),
    #[error("read store file error: {0}")]
    ReadStoreError(sled::Error),
    #[error("serde json error: {0}")]
    SerdeJsonError(serde_json::Error),
    #[error("invalid string head")]
    InvalidStringHead(actix_web::http::header::ToStrError),
    #[error("invalid writer key")]
    InvalidWriterKey,
    #[error("invalid reader key")]
    InvalidReaderKey,
    #[error("Cound not read remote ip addr")]
    ReadIpAddrError,
    #[error("blocking error: {0}")]
    BlockingError(Box<actix_web::error::BlockingError<Error>>),
}

impl actix_web::error::ResponseError for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::ReadConfigError(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Error {
        Error::ParseConfigError(err)
    }
}

impl From<sled::Error> for Error {
    fn from(err: sled::Error) -> Error {
        Error::ReadStoreError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::SerdeJsonError(err)
    }
}

impl From<actix_web::http::header::ToStrError> for Error {
    fn from(err: actix_web::http::header::ToStrError) -> Error {
        Error::InvalidStringHead(err)
    }
}

impl From<actix_web::error::BlockingError<Error>> for Error {
    fn from(err: actix_web::error::BlockingError<Error>) -> Error {
        Error::BlockingError(Box::new(err))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
