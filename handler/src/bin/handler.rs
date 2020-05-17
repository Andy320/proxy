#[macro_use]
extern crate log;

use tokio::signal;
use handler::grpc::server as gs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = tokio::sync::oneshot::channel::<()>();

    tokio::spawn(async move {
        if let Ok(()) = signal::ctrl_c().await {
            let _ = tx.send(());
        };
    });

    match gs::start_grpc_server(async move {
        rx.await.ok();
    }).await {
        Ok(()) => info!("grpc shutdown successfully"),
        Err(e) => error!("grpc error occurs: {:#?}", e),
    }

    Ok(())
}