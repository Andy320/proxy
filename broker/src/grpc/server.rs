use tonic::{transport::Server, Request, Response, Status};
use proto::pb::{Message, MessageType, Entry, EntryType, broker_server::{Broker, BrokerServer}};

use futures::Future;
use proto::util;

pub async fn start_grpc_server<F: Future<Output = ()>>(signal: F) -> crate::error::Result<()> {
    let addr = "0.0.0.0:9000".parse().unwrap();
    info!("GRPC Server listening on {}", addr);
    Server::builder()
        .add_service(BrokerServer::new(MyBusiness::default()))
        .serve_with_shutdown(addr, signal)
        .await?;

    Ok(())
}

#[derive(Default)]
pub struct MyBusiness {}

#[tonic::async_trait]
impl Broker for MyBusiness {
    async fn push(
        &self,
        request: Request<Message>,
    ) -> Result<Response<Message>, Status> {
        info!("Got a grpc request from {:?}, its type is {:#?}", request.remote_addr(), request.into_inner().msg_type);
        let s = "broker said: hi guys, good afternoon";
        let m = util::new_msg(MessageType::MsgRequestPushResponse, s);
        Ok(Response::new(m))
    }
}

