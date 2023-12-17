use crate::{{crate_name}}::{{crate_name}}_service_server::{{crate_name | pascal_case}}ServiceServer;
use crate::{{crate_name}}::{{crate_name}}_service_server::{{crate_name | pascal_case}}Service;
use crate::{{crate_name}}::{{crate_name | pascal_case}}Request;
use crate::{{crate_name}}::{{crate_name | pascal_case}}Response;
use std::error::Error;
use tonic::{Request, Response, Status};
use tonic_reflection::server::{Builder, ServerReflection, ServerReflectionServer};

const FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("reflection_descriptor.bin");

#[derive(Default)]
pub struct {{crate_name | pascal_case}}ServiceImpl {}

impl {{crate_name | pascal_case}}ServiceImpl {
    pub fn new() -> {{crate_name | pascal_case}}ServiceServer<{{crate_name | pascal_case}}ServiceImpl> {
        {{crate_name | pascal_case}}ServiceServer::new({{crate_name | pascal_case}}ServiceImpl {})
    }
    pub fn attach_reflection() -> Result<ServerReflectionServer<impl ServerReflection + 'static>, Box<dyn Error>> {
        Builder::configure()
            .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
            .build()
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}

#[tonic::async_trait]
impl {{crate_name | pascal_case}}Service for {{crate_name | pascal_case}}ServiceImpl {
    async fn {{rpc_method}}(&self, request: Request<{{crate_name | pascal_case}}Request>) -> Result<Response<{{crate_name | pascal_case}}Response>, Status> {
        let name = request.into_inner().name;
        let reply = {{crate_name | pascal_case}}Response {
            message: format!("Hello {}!", name),
        };
        Ok(Response::new(reply))
    }
}
