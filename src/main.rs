mod _struct;
mod config;
mod csrf_token;
mod fetch;
mod handler;
mod utils;
use axum::{Router, routing::get};
use tokio::sync::watch;

use crate::csrf_token::update_csrf_token;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let (shutdown_sender, mut shutdown_receiver) = watch::channel(false);
    tokio::spawn(update_csrf_token(shutdown_sender));
    let shutdown_signal = async move {
        while shutdown_receiver.changed().await.is_ok() {
            if *shutdown_receiver.borrow() {
                println!("Shutdown signal received from background task!");
                break;
            }
        }
    };

    let app = Router::new().route("/{user_name}", get(handler::get_ig_detail));
    let host = format!("{}:{}", config::HOST_URL, config::HOST_PORT);
    println!("{}",host);
    let listener = tokio::net::TcpListener::bind(&host)
        .await
        .map_err(|e| println!("listen err {}",e))?;
    println!("Listening on {}", host);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal)
        .await
        .map_err(|_| println!("serve err"))?;
    Ok(())
}
