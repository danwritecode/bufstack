use tonic::{Request, Response, Status};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

use hello_world::greeter_server::Greeter;
use hello_world::{HelloReply, HelloRequest};

#[derive(Debug, Default)]
pub struct HelloWorldServiceImpl {}

#[tonic::async_trait]
impl Greeter for HelloWorldServiceImpl {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let request = request.into_inner();

        let reply = HelloReply {
            message: format!(
                "Hello {} from Bufstack! Your message: {}",
                request.name,
                request.message
            ),
        };

        Ok(Response::new(reply))
    }
}
