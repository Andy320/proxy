use tonic::{transport::Server, Request, Response, Status};
use proto::pb::{Message, MessageType, Entry, EntryType, broker_server::{Broker, BrokerServer}};
use futures::Future;
use proto::util;
use std::collections::HashMap;
use crate::grpc::client::{Router, Builder};
use std::sync::{Arc, Mutex};
use std::sync::RwLock;

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
pub struct MyBusiness {
    endpoints: Arc<RwLock<HashMap<String, Router>>>,
    redis_client: redis::Client,
}

impl Default for MyBusiness {
    fn default() -> Self {
        endpoints = Arc::new(RwLock::new(HashMap::new()));
    }
}

impl MyBusiness {
    fn get_router(self, end_point: &str) -> Option<Router> {
        let r = self.endpoints.read().unwrap();
        r.get(end_point)
    }

    fn delete_router(mut self, end_point: &str) -> Option<Router> {
        let mut w = self.endpoints.write().unwrap();
        w.remove(end_point)
    }

    async fn update_router(mut self, end_point: &str) -> crate::error::Result<Router> {
        if self.get_router(end_point).is_some() {
            self.delete_router(end_point);
        }
        let r: Router = Builder::new().build_from_ep(String::from(end_point))?;
        let mut w = self.endpoints.write().unwrap();
        w.insert(String::from(end_point), r);
        Ok(r)
    }

    fn send() {

    }
}


#[tonic::async_trait]
impl Broker for MyBusiness {
    async fn push(
        &self,
        request: Request<Message>,
    ) -> Result<Response<Message>, Status> {
        info!("Got a grpc request from {:?}, its type is {:#?}", request.remote_addr(), request.into_inner().msg_type);

        let entry = util::get_entry0(request.into_inner());
        match util::get_entry0_context(message) {
            Some(user) => {

                match util::get_entry0_data(message) {
                    Some(content) => {
                        let m = util::new_msg(MessageType::MsgRequestSendOut, content.as_str());
                        let request = tonic::Request::new(m);
                        // from redis
                        self.get_router()
                    },
                    None => {},
                }
            },
            None => {},
        }

        let s = "marketing said: push it out now";
        let m = util::new_msg(MessageType::MsgRequestPushResponse, s);

        Ok(Response::new(m))
    }

    async fn online(
        &self,
        request: Request<Message>,
    ) -> Result<Response<Message>, Status> {
        info!("Got a grpc request from {:?}, its type is {:#?}", request.remote_addr(), request.into_inner().msg_type);
        let s = "broker said: I will check his kept messages now";
        let m = util::new_msg(MessageType::MsgRequestOnlineResponse, s);
        Ok(Response::new(m))
    }
}