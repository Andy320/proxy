#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref BROKER_HOSTNAME: String = {
        match std::env::var("BROKER_HOSTNAME_KEY") {
            Ok(val) => val,
            Err(_e) => "broker".to_string(),
        }
    };
}

pub mod grpc;
pub mod error;