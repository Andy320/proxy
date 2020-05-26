use proto::pb::{Message, MessageType, Entry, EntryType};
use proto::pb::router_client::RouterClient;
use tonic::transport::{Channel, Endpoint};
use tonic::{metadata::MetadataValue, Code, Request, Response, Status};
use crate::error;
use proto::util;
use tonic::codegen::{Context, Pin, Poll};
use std::collections::HashMap;

pub type Router = RouterClient<Channel>;

#[derive(Debug, Clone)]
pub struct Builder;

impl Builder {
    pub fn new() -> Builder {
        Builder
    }

    pub async fn build_from_ep(&mut self, end_point: String) -> error::Result<Router> {
        let mut grpc_addr = String::from("http://");
        grpc_addr.push_str(end_point.as_str());
        grpc_addr.push_str(":9000");
        info!("directly connected grpc server url is {}", grpc_addr);
        let endpoint = Endpoint::from_shared(grpc_addr)
            .expect("failed to be converted into a uri")
            .timeout(*common::GRPC_TIMEOUT)
            .concurrency_limit(*common::GRPC_CONCURRENCY);
        let channel = endpoint.connect().await?;
        let b = Router::new(channel.clone());
        Ok(b)
    }
}


