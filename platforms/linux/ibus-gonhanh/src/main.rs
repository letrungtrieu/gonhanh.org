//! IBus engine for Gõ Nhanh Vietnamese Input Method
//!
//! This implements the IBus D-Bus interface to provide Vietnamese input.

mod engine;
mod ffi;
mod keycode;

use zbus::connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    log::info!("Starting Gõ Nhanh IBus engine...");

    // Initialize gonhanh_core
    ffi::initialize();
    log::info!("Gonhanh core initialized");

    // Create D-Bus connection
    let _connection = connection::Builder::session()?
        .name("org.freedesktop.IBus.GoNhanh")?
        .serve_at("/org/freedesktop/IBus/Engine/GoNhanh", engine::GoNhanhEngine::new())?
        .build()
        .await?;

    log::info!("IBus engine registered on D-Bus");
    log::info!("Engine ready, waiting for connections...");

    // Keep the service running
    std::future::pending::<()>().await;

    Ok(())
}
