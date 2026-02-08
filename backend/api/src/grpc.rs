use tokio::net::TcpListener;
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;

use crate::services::helloworld_service::hello_world::greeter_server::GreeterServer;
use crate::services::HelloWorldServiceImpl;

// To enable Clerk auth, uncomment:
// use std::sync::Arc;
// use tonic_middleware::RequestInterceptorLayer;
// use crate::middleware::auth::{AuthInterceptor, AuthServiceImpl};

pub mod middleware;
pub mod services;

const MAX_PORT_ATTEMPTS: u16 = 10;

async fn bind_listener(start_port: u16) -> Result<(TcpListener, u16), Box<dyn std::error::Error>> {
    for offset in 0..MAX_PORT_ATTEMPTS {
        let port = start_port + offset;
        match TcpListener::bind(format!("0.0.0.0:{}", port)).await {
            Ok(listener) => return Ok((listener, port)),
            Err(e) if offset < MAX_PORT_ATTEMPTS - 1 => {
                tracing::warn!("Port {} unavailable ({}), trying {}...", port, e, port + 1);
            }
            Err(e) => return Err(format!("No available port found after {} attempts: {}", MAX_PORT_ATTEMPTS, e).into()),
        }
    }
    unreachable!()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt().with_env_filter("info").init();

    let (listener, port) = bind_listener(50051).await?;
    let incoming = tokio_stream::wrappers::TcpListenerStream::new(listener);

    // Initialize database
    let db_config = data::DatabaseConfig::from_env();
    let pool = data::create_pool(&db_config).await?;
    data::run_migrations(&pool).await?;

    // Initialize services
    let greeter_service = HelloWorldServiceImpl::default();

    // To enable Clerk auth, uncomment:
    // let auth_interceptor = AuthInterceptor {
    //     auth_service: Arc::new(AuthServiceImpl::new()),
    // };

    tracing::info!("gRPC server listening on 0.0.0.0:{}", port);

    Server::builder()
        .accept_http1(true)
        .layer(GrpcWebLayer::new())
        // .layer(RequestInterceptorLayer::new(auth_interceptor.clone()))
        .add_service(GreeterServer::new(greeter_service))
        .serve_with_incoming(incoming)
        .await?;

    Ok(())
}
