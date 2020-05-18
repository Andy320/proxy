#[macro_use]
extern crate log;

use log4rs;
use router::grpc::client::{GrpcClientBuilder, GrpcClient};

#[actix_rt::main]
async fn main() {
    log4rs::init_file("/etc/router/log4rs.yaml", Default::default()).unwrap();
    match GrpcClientBuilder::new().build().await {
        Ok(gc) => {
            info!("connect grpc server successfully");
            match router::http::start(gc).await {
                Ok(()) => info!("server shutdown successfully"),
                Err(e) => error!("server error occurs: {:#?}", e),
            }
        }
        Err(e) => error!("failed to connect grpc server {:#?}", e),
    }
}