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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt().with_env_filter("info").init();

    let port: u16 = std::env::var("GRPC_PORT")
        .unwrap_or_else(|_| "50051".to_string())
        .parse()?;
    let addr = format!("0.0.0.0:{}", port).parse()?;

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
        .serve(addr)
        .await?;

    Ok(())
}
