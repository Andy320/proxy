#[macro_use]
extern crate log;

use router::grpc::client::{GrpcClientBuilder, GrpcClient};

#[actix_rt::main]
async fn main() {
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