use proto::pb::{Message, MessageType, Entry, EntryType};
use proto::pb::handler_client::HandlerClient;
use tonic::transport::{Channel, Endpoint};
use tonic::{metadata::MetadataValue, Code, Request, Response, Status};
use crate::error;
use proto::util;

type TonicClient = HandlerClient<Channel>;

#[derive(Debug, Clone)]
pub struct GrpcClientBuilder {}

impl GrpcClientBuilder {
    pub fn new() -> GrpcClientBuilder {
        GrpcClientBuilder {}
    }

    pub async fn build(&self) -> error::Result<GrpcClient> {
        let mut grpc_addr = String::from("http://");
        grpc_addr.push_str(&*crate::HANDLER_HOSTNAME);
        grpc_addr.push_str(".");
        grpc_addr.push_str(&*crate::SERVER_DOMAIN);
        grpc_addr.push_str(":9000/handler");
        info!("grpc url is {}", grpc_addr);
        let endpoint = Endpoint::from_shared(grpc_addr)
            .expect("failed to be converted into a uri")
            .timeout(*crate::GRPC_TIMEOUT)
            .concurrency_limit(*crate::GRPC_CONCURRENCY);
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
    pub async fn heart_beaten(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let str = "hello, handler";
        let m = util::new_msg(MessageType::MsgRequestHeartBeaten, str);
        let request = tonic::Request::new(m);
        let response = self.tc.heart_beaten(request).await?;
        // debug!("RESPONSE={:?}", response);
        if let Some(s) = util::get_entry0(response.into_inner()) {
            debug!("{}", s)
        }

        Ok(())
    }
}