use thiserror::Error;
use std::{io, result};
// use std::backtrace::Backtrace;

pub type Result<T> = result::Result<T, RouterError>;

#[derive(Error, Debug)]
pub enum RouterError {
    #[error("cannot bind listening address")]
    IOFailure {
        #[from]
        source: io::Error,
        // backtrace: Backtrace,
    },
    #[error("invalid input")]
    InvalidFailure{
        #[from]
        source: validator::ValidationErrors,
        // backtrace: Backtrace,
    },
    #[error("failed to create token")]
    JWSFailure{
        #[from]
        source: jsonwebtoken::errors::Error,
        // backtrace: Backtrace,
    },
    #[error("actix error")]
    ActixHttpFailure{
        #[from]
        source: actix_http::error::Error,
        // backtrace: Backtrace,
    },
    #[error("tonic transport error")]
    GRPCTransportFailure{
        #[from]
        source: tonic::transport::Error,
        // backtrace: Backtrace,
    },
}
