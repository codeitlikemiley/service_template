use crate::{{service_name}}::{{service_name}}_service_server::{{service_name | pascal_case}}ServiceServer;
use crate::{{service_name}}::{{"{"}}{{service_name}}_service_server::{{service_name | pascal_case}}Service, {{service_name | pascal_case}}Request, {{service_name | pascal_case}}Response};
use std::error::Error;
use tonic::{Request, Response, Status};
use tonic_reflection::server::{Builder, ServerReflection, ServerReflectionServer};

const FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("{{service_name}}_reflection_descriptor.bin");

#[derive(Default)]
pub struct {{service_name | pascal_case}}ServiceImpl {}

impl {{service_name | pascal_case}}ServiceImpl {
    pub fn new() -> {{service_name | pascal_case}}ServiceServer<{{service_name | pascal_case}}ServiceImpl> {
        {{service_name | pascal_case}}ServiceServer::new({{service_name | pascal_case}}ServiceImpl {})
    }
    pub fn attach_reflection() -> Result<ServerReflectionServer<impl ServerReflection + 'static>, Box<dyn Error>> {
        Builder::configure()
            .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
            .build()
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}

#[tonic::async_trait]
impl {{service_name | pascal_case}}Service for {{service_name | pascal_case}}ServiceImpl {
    async fn {{rpc_method}}(&self, request: Request<{{service_name | pascal_case}}Request>) -> Result<Response<{{service_name | pascal_case}}Response>, Status> {
        let name = request.into_inner().name;
        let reply = {{service_name | pascal_case}}Response {
            message: format!("Hello {}!", name),
        };
        Ok(Response::new(reply))
    }
}
