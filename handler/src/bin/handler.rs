#[macro_use]
extern crate log;

use log4rs;
use tokio::signal;
use handler::grpc::server as hgs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file("/etc/handler/log4rs.yaml", Default::default()).unwrap();
    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    tokio::spawn(async move {
        if let Ok(()) = signal::ctrl_c().await {
            info!("shutting down handler's grpc server...");
            let _ = tx.send(());
        };
    });
    match hgs::start_grpc_server(async move {
        info!("grpc server started successfully");
        rx.await.ok();
    }).await {
        Ok(()) => info!("grpc server shutdown successfully"),
        Err(e) => error!("grpc server error occurs: {:#?}", e),
    }

    Ok(())
}