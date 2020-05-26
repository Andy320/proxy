use thiserror::Error;
use std::{io, result};
use std::sync::PoisonError;
// use std::backtrace::Backtrace;

pub type Result<T> = result::Result<T, BrokerError>;

#[derive(Error, Debug)]
pub enum BrokerError {
    #[error("cannot bind listening address")]
    IOFailure {
        #[from]
        source: io::Error,
        // backtrace: Backtrace,
    },

    #[error("tonic transport error")]
    GRPCTransportFailure {
        #[from]
        source: tonic::transport::Error,
        // backtrace: Backtrace,
    },

    #[error("cannot create the connection to grpc server")]
    AddRouterFailure,

    #[error("cannot delete the connection to grpc server")]
    DeleteRouterFailure,

    #[error("tonic transport error")]
    AAA {
        #[from]
        source: PoisonError<T>,
        // backtrace: Backtrace,
    },
}
