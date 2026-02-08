use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_env_filter("info").init();
    tracing::info!("Placeholder worker started");

    loop {
        tracing::info!("Worker heartbeat");
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
