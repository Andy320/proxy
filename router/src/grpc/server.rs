use tonic::{transport::Server, Request, Response, Status};
use crate::pb::{prelude, greeter_server::{Greeter, GreeterServer}};
use futures::Future;
use crate::util;

pub async fn start_grpc_server<F: Future<Output = ()>>(signal: F) -> crate::error::Result<()> {
    let addr = "0.0.0.0:9000".parse().unwrap();
    info!("GRPC Server listening on {}", addr);
    Server::builder()
        .add_service(GreeterServer::new(MyGreeter::default()))
        .serve_with_shutdown(addr, signal)
        .await?;

    Ok(())
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<Message>,
    ) -> Result<Response<Message>, Status> {
        info!("Got a request from {:?}", request.remote_addr());
        let s = "good morning";
        let m = util::new_msg(MessageType::MsgRequestHelloResponse, s);
        Ok(Response::new(m))
    }
}

