#[macro_use]
extern crate log;

use log4rs;
use tokio::signal;
// use broker::grpc::server as gs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file("/etc/broker/log4rs.yaml", Default::default()).unwrap();
    let (tx, rx) = tokio::sync::oneshot::channel::<()>();

    // tokio::spawn(async move {
    //     if let Ok(()) = signal::ctrl_c().await {
    //         let _ = tx.send(());
    //     };
    // });
    info!("broker server started successfully");
    if let Ok(()) = signal::ctrl_c().await {
        let _ = tx.send(());
    }
    rx.await.ok();
    info!("broker server shut down successfully");
    // match gs::start_grpc_server(async move {
    //     rx.await.ok();
    // }).await {
    //     Ok(()) => info!("grpc shutdown successfully"),
    //     Err(e) => error!("grpc error occurs: {:#?}", e),
    // }

    Ok(())
}