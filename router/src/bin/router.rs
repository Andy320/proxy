#[macro_use]
extern crate log;

use tokio::signal;
use log4rs;
use router::grpc::client::{GrpcClientBuilder, GrpcClient};
use router::grpc::server as rgs;

#[actix_rt::main]
async fn main() {
    log4rs::init_file("/etc/router/log4rs.yaml", Default::default()).unwrap();

    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    tokio::spawn(async move {
        if let Ok(()) = signal::ctrl_c().await {
            info!("shutting down router's grpc server...");
            let _ = tx.send(());
        };
    });
    tokio::spawn(async move {
        match rgs::start_grpc_server(async move {
            info!("grpc server started successfully");
            rx.await.ok();
        }).await {
            Ok(()) => info!("grpc server shutdown successfully"),
            Err(e) => error!("grpc server error occurs: {:#?}", e),
        }
    });

    match GrpcClientBuilder::new().build().await {
        Ok(gc) => {
            info!("connect handler's grpc server successfully");
            match router::http::start(gc).await {
                Ok(()) => info!("shutting down router's http server successfully"),
                Err(e) => error!("http server error occurs: {:#?}", e),
            }
        }
        Err(e) => error!("failed to connect handler's grpc server {:#?}", e),
    }
}