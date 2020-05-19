use tonic::{transport::Server, Request, Response, Status};
use proto::pb::{Message, MessageType, Entry, EntryType, handler_server::{Handler, HandlerServer}};

use futures::Future;
use proto::util;

pub async fn start_grpc_server<F: Future<Output = ()>>(signal: F) -> crate::error::Result<()> {
    let addr = "0.0.0.0:9000".parse().unwrap();
    info!("GRPC Server listening on {}", addr);
    Server::builder()
        .add_service(HandlerServer::new(MyBusiness::default()))
        .serve_with_shutdown(addr, signal)
        .await?;

    Ok(())
}

#[derive(Default)]
pub struct MyBusiness {}

#[tonic::async_trait]
impl Handler for MyBusiness {
    async fn heart_beaten(
        &self,
        request: Request<Message>,
    ) -> Result<Response<Message>, Status> {
        info!("Got a grpc request from {:?}, its type is {:#?}", request.remote_addr(), request.into_inner().msg_type);
        let s = "handler said: hi router, good morning";
        let m = util::new_msg(MessageType::MsgRequestHeartBeatenResponse, s);
        Ok(Response::new(m))
    }
}

