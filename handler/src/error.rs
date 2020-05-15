use thiserror::Error;
use std::{io, result};
// use std::backtrace::Backtrace;
use std::error::Error;

pub type Result<T> = result::Result<T, HandlerError>;

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error("cannot bind listening address")]
    IOFailure {
        #[from]
        source: io::Error,
        // backtrace: Backtrace,
    },
}
