use std::sync::Arc;
use clerk_rs::clerk::Clerk;
use clerk_rs::validators::authorizer::validate_jwt;
use clerk_rs::validators::jwks::MemoryCacheJwksProvider;
use clerk_rs::ClerkConfiguration;

use tonic::body::Body;
use tonic::codegen::http::{HeaderValue, Request};
use tonic::{async_trait, Status};
use tonic_middleware::RequestInterceptor;

use axum_extra::extract::cookie::CookieJar;

#[async_trait]
pub trait AuthService: Send + Sync + 'static {
    async fn verify_token(&self, token: &str) -> Result<String, String>;
}

#[derive(Clone)]
pub struct AuthServiceImpl {
    pub provider: Arc<MemoryCacheJwksProvider>
}

impl AuthServiceImpl {
    pub fn new() -> Self {
        let clerk_key = std::env::var("NUXT_CLERK_SECRET_KEY").expect("CLERK_SECRET_KEY not set");
        let config = ClerkConfiguration::new(None, None, Some(clerk_key), None);
        let clerk = Clerk::new(config);
        let provider = Arc::new(MemoryCacheJwksProvider::new(clerk));

        AuthServiceImpl { provider }
    }
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn verify_token(&self, token: &str) -> Result<String, String> {
        match validate_jwt(token, self.provider.clone()).await {
            Ok(jwt) => {
                Ok(jwt.sub)
            },
            Err(e) => {
                tracing::error!("An error occured validation the JWT from clerk: {:?}", e);
                Err("Unauthenticated".to_string())
            }
        }
    }
}

#[derive(Clone)]
pub struct AuthInterceptor<A: AuthService> {
    pub auth_service: Arc<A>,
}

#[async_trait]
impl<A: AuthService> RequestInterceptor for AuthInterceptor<A> {
    async fn intercept(&self, mut req: Request<Body>) -> Result<Request<Body>, Status> {
        let cookies = CookieJar::from_headers(req.headers());

        match cookies.get("__session") {
            Some(token) => {
                // Get user id from the token
                let token = token.value();

                let user_id = self
                    .auth_service
                    .verify_token(token).await
                    .map_err(Status::unauthenticated)?;

                // Set user id in header, so it can be used in grpc services through tonic::Request::metadata()
                let user_id_header_value = HeaderValue::from_str(&user_id.to_string())
                    .map_err(|_e| Status::internal("Failed to convert user_id to header value"))?;

                req.headers_mut().insert("user_id", user_id_header_value);
                Ok(req)
            }
            _ => {
                tracing::error!("No __session cookie found");
                Err(Status::unauthenticated("Unauthenticated"))
            }
        }
    }
}
