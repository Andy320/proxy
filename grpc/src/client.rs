use crate::pb::{Message, MessageType, Entry, EntryType};
use crate::pb::greeter_client::GreeterClient;
use tonic::transport::{Channel, Endpoint};
use tonic::{metadata::MetadataValue, Code, Request, Response, Status};
use crate::{error, util};
use std::time::Duration;
use std::borrow::Borrow;

type TonicClient = GreeterClient<Channel>;

lazy_static! {
    pub static ref SERVER_DOMAIN: String = {
        match std::env::var("SERVICES_DOMAIN_KEY") {
            Ok(val) => val,
            Err(e) => "default.svc.cluster.local".to_string(),
        }
    };
    pub static ref HANDLER_HOSTNAME: String = {
        match std::env::var("HANDLER_HOSTNAME_KEY") {
            Ok(val) => val,
            Err(e) => "handler".to_string(),
        }
    };
    pub static ref GRPC_TIMEOUT: Duration = {
        match std::env::var("GRPC_TIMEOUT_KEY") {
            Ok(val) => {
                let t = val.parse::<u64>().expect("grpc_timeout is invalid");
                std::time::Duration::from_secs(t)
            },
            Err(e) => std::time::Duration::from_secs(5),
        }
    };
    pub static ref GRPC_CONCURRENCY: usize = {
        match std::env::var("GRPC_CONCURRENCY_KEY") {
            Ok(val) => val.parse::<usize>().expect("grpc_concurrency is invalid"),
            Err(e) => 30,
        }
    };
}

#[derive(Debug, Clone)]
pub struct GrpcClientBuilder {}

impl GrpcClientBuilder {
    pub fn new() -> GrpcClientBuilder {
        GrpcClientBuilder {}
    }

    pub async fn build(&self) -> error::Result<GrpcClient> {
        let mut grpc_addr = String::from("http://");
        grpc_addr.push_str(&*HANDLER_HOSTNAME);
        grpc_addr.push_str(".");
        grpc_addr.push_str(&*SERVER_DOMAIN);
        grpc_addr.push_str(":9000/handler");
        let mut endpoint = Endpoint::from_shared(grpc_addr)
            .expect("failed to be converted into a uri")
            .timeout(*GRPC_TIMEOUT)
            .concurrency_limit(*GRPC_CONCURRENCY);
        let channel = endpoint.connect().await?;
        let tc = TonicClient::new(channel.clone());
        Ok(GrpcClient { tc })
    }
}

#[derive(Debug, Clone)]
pub struct GrpcClient {
    tc: TonicClient,
}

impl GrpcClient {
    pub async fn say_hello(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let str = "hello, grpc";

        let m = util::new_msg(MessageType::MsgRequestHello, str);

        let request = tonic::Request::new(m);

        let response = self.tc.say_hello(request).await?;

        debug!("RESPONSE={:?}", response);

        if let Some(s) = util::get_entry0(response.into_inner()) {
            debug!("it said: {}", s)
        }

        Ok(())
    }
}



