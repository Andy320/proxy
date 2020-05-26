use tonic::{transport::Server, Request, Response, Status};
use proto::pb::{Message, MessageType, Entry, EntryType, router_server::{Router, RouterServer}};
use futures::Future;
use proto::util;

pub async fn start_grpc_server<F: Future<Output = ()>>(signal: F) -> crate::error::Result<()> {
    let addr = "0.0.0.0:9000".parse().unwrap();
    info!("GRPC Server listening on {}", addr);
    Server::builder()
        .add_service(RouterServer::new(MyBusiness::default()))
        .serve_with_shutdown(addr, signal)
        .await?;

    Ok(())
}

#[derive(Default)]
pub struct MyBusiness {}

#[tonic::async_trait]
impl Router for MyBusiness {
    async fn send_out(
        &self,
        request: Request<Message>,
    ) -> Result<Response<Message>, Status> {
        debug!("Got a grpc request from {:?}, its type is {:#?}", request.remote_addr(), request.into_inner().msg_type);
        let s = "router said: hi broker, good morning";
        let m = util::new_msg(MessageType::MsgRequestSendOutResponse, s);
        Ok(Response::new(m))
    }
}

