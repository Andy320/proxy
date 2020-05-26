use proto::pb::{Message, MessageType, Entry, EntryType};
use proto::pb::handler_client::HandlerClient;
use tonic::transport::{Channel, Endpoint};
use tonic::{metadata::MetadataValue, Code, Request, Response, Status};
use crate::error;
use proto::util;
use tonic::codegen::{Context, Pin, Poll};
use actix::ActorFuture;

pub type Handler = HandlerClient<Channel>;

#[derive(Debug, Clone)]
pub struct Builder;

impl Builder {
    pub fn new() -> Builder {
        Builder
    }

    pub async fn build(&self) -> error::Result<Handler> {
        let mut grpc_addr = String::from("http://");
        grpc_addr.push_str(&*crate::HANDLER_HOSTNAME);
        grpc_addr.push_str(".");
        grpc_addr.push_str(&*common::SERVER_DOMAIN);
        grpc_addr.push_str(":9000/handler");
        info!("grpc url is {}", grpc_addr);
        let endpoint = Endpoint::from_shared(grpc_addr)
            .expect("failed to be converted into a uri")
            .timeout(*common::GRPC_TIMEOUT)
            .concurrency_limit(*common::GRPC_CONCURRENCY);
        let channel = endpoint.connect().await?;
        let h = Handler::new(channel.clone());
        Ok(h)
    }
}