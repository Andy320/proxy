#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate clap;
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
            Err(e) => std::time::Duration::from_secs(20),
        }
    };
    pub static ref WS_HEARTBEAT: std::time::Duration = {
        match std::env::var("WS_HEARTBEAT_KEY") {
            Ok(val) => {
                let t = val.parse::<u64>().expect("ws_timeout is invalid");
                std::time::Duration::from_secs(t)
            },
            Err(e) => std::time::Duration::from_secs(5),
        }
    };
}

pub mod router;
pub mod error;
pub mod token;