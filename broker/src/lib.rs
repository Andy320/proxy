#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref ROUTER_HOSTNAME: String = {
        match std::env::var("ROUTER_HOSTNAME_KEY") {
            Ok(val) => val,
            Err(_e) => "router".to_string(),
        }
    };

}

pub mod grpc;
mod error;