use thiserror::Error;
use std::{io, result};
// use std::backtrace::Backtrace;

pub type Result<T> = result::Result<T, HandlerError>;

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error("cannot bind listening address")]
    IOFailure {
        #[from]
        source: io::Error,
        // backtrace: Backtrace,
    },
    #[error("tonic transport error")]
    GRPCTransportFailure{
        #[from]
        source: tonic::transport::Error,
        // backtrace: Backtrace,
    },
}
