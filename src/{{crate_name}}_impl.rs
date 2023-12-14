use crate::{{crate_name}}::{hello_service_server::HelloService, HelloRequest, HelloResponse};
use tonic::{Request, Response, Status};

pub struct {{CrateName}}ServiceImpl {}

#[tonic::async_trait]
impl HelloService for {{CrateName}}ServiceImpl {
async fn hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloResponse>, Status> {
    let name = request.into_inner().name;
    let reply = HelloResponse {
        message: format!("Hello {}!", name),
    };
    Ok(Response::new(reply))
}
}
