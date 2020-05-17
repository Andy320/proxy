#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref HTTP_ADDR: &'static str = {
        "0.0.0.0:9080"
    };
    pub static ref WS_TIMEOUT: std::time::Duration = {
        match std::env::var("WS_TIMEOUT_KEY") {
            Ok(val) => {
                let t = val.parse::<u64>().expect("ws_timeout is invalid");
                std::time::Duration::from_secs(t)
            },
            Err(_e) => std::time::Duration::from_secs(20),
        }
    };
    pub static ref WS_HEARTBEAT: std::time::Duration = {
        match std::env::var("WS_HEARTBEAT_KEY") {
            Ok(val) => {
                let t = val.parse::<u64>().expect("ws_timeout is invalid");
                std::time::Duration::from_secs(t)
            },
            Err(_e) => std::time::Duration::from_secs(5),
        }
    };

    pub static ref SERVER_DOMAIN: String = {
        match std::env::var("SERVICES_DOMAIN_KEY") {
            Ok(val) => val,
            Err(_e) => "default.svc.cluster.local".to_string(),
        }
    };
    pub static ref HANDLER_HOSTNAME: String = {
        match std::env::var("HANDLER_HOSTNAME_KEY") {
            Ok(val) => val,
            Err(_e) => "handler".to_string(),
        }
    };
    pub static ref GRPC_TIMEOUT: std::time::Duration = {
        match std::env::var("GRPC_TIMEOUT_KEY") {
            Ok(val) => {
                let t = val.parse::<u64>().expect("grpc_timeout is invalid");
                std::time::Duration::from_secs(t)
            },
            Err(_e) => std::time::Duration::from_secs(5),
        }
    };
    pub static ref GRPC_CONCURRENCY: usize = {
        match std::env::var("GRPC_CONCURRENCY_KEY") {
            Ok(val) => val.parse::<usize>().expect("grpc_concurrency is invalid"),
            Err(_e) => 30,
        }
    };
}

pub mod http;
pub mod error;
pub mod token;
pub mod grpc;