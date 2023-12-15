# GRPC Service Generator

[![Build Template](https://github.com/codeitlikemiley/service_template/actions/workflows/build.yml/badge.svg)](https://github.com/codeitlikemiley/service_template/actions/workflows/build.yml)
## Requirements

Note: Install the following before you can use the template generator.

- [rust, rustup, cargo](https://www.rust-lang.org/tools/install)
- [cargo-generate](https://cargo-generate.github.io/cargo-generate/installation.html)
- [protoc](https://grpc.io/docs/protoc-installation/)

## GRPC Client

- [grpcurl](https://github.com/fullstorydev/grpcurl)
- [evans_cli](https://github.com/ktr0731/evans)
- [Postman](https://www.postman.com/downloads/)

## Project Structure

```sh
workspace (root)
├── Cargo.toml
├── Carg.lock
├── README.md
│
├── backend
│
├── frontend (Use Dioxus Template)
│
└── services
    └── <service_name>
        ├── proto
        │  └── <service_name>.proto
        ├── src
        │    ├── <service_name.rs> (generated with cargo build)
        │    ├── <service_name_impl.rs>
        │    └── lib.rs
        ├── cargo-generate.toml
        └── Cargo.toml

```

## Usage

### Change directory to the services directory.

Note: The following command will generate a new GRPC service template.

```bash
cd <workspace_root>/services
```
### Generate a new GRPC service template.

For more info how to use cargo-generate check this [link](https://cargo-generate.github.io/cargo-generate/usage.html)

Learn about Liquid Templating [here](https://shopify.github.io/liquid/)


```sh
cargo generate --git codeitlikemiley/services_template --name <service_name>

🔧   Destination: /Users/uriah/Code/example
🔧   project-name: example ...
🔧   Generating template ...
🤷   Enter a brief description of the service: [default: Default Service Description]: Default Service Description
🤷   Enter the RPC method name: [default: make]: make
🔧   Moving generated files into: `/Users/uriah/Code/example`...
🔧   Initializing a fresh Git repository
✨   Done! New project created /Users/uriah/Code/example


cd <service_name>

```

### Build , Testing and Documentation

```sh
cargo build
cargo test
cargo doc --open
```

### Configure your rust tonic service

To learn more about Rust Tonic click this [link](https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md)

### Add to your Rust App workspace `Cargo.toml`

```toml
[workspace]
resolver = "2"
members = [
  "backend", # consumes all the services in the services directory
  # add here the new service
  "services/<service_name>
]
# if you download this repo and wanna use locally , you can exclude it on the workspace
# git clone codeitlikemiley/services_template template
# note you need to also add it on your .gitignore
exclude = ["template"]
```

### Update your server `main.rs` and `Cargo.toml`

`Cargo.toml`
```toml
[package]
name = "server"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
auth_service = { path = "../services/auth" }
tonic = "0.10.2"
tokio = { version = "1.35.0", features = [] }
```

`main.rs`
```rust
use tonic::{transport::Server};
// Import ServiceNameServer
use auth_service::auth::auth_service_server::{AuthServiceServer};

use auth_service::auth_impl::AuthServiceImpl;

#[tokio::main] // Use the tokio runtime for async
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    
    // Create a new instance of your service implementation
    let auth = AuthServiceImpl{};

    Server::builder()
        // Add the service to the gRPC server
        .add_service(AuthServiceServer::new(auth))
        // Add more services here
        .serve(addr)
        .await?;

    Ok(())
}
```

### Test your Service with GRPC Client

1. Run Server
```sh
cargo run -p server
```

2. Use `grpcurl` Client

Tp invoke the specific rpc method use the following command
```sh
grpcurl -plaintext -import-path ./services/auth/proto -proto auth.proto -d '{"name": "Uriah"}' '[::1]:50051' auth.AuthService.Login
```

output:
```json
{
  "message": "Hello Uriah!"
}
```


Note: that the example `auth.AuthService.Login` is the `package_name.ServiceName.MethodName` from the `auth.proto` file

`auth.proto`
```protobuf
package auth;

service AuthService {
  rpc Login (LoginRequest) returns (LoginResponse) {}
}
```

## Development

```bash
git clone https://github.com/codeitlikemiley/service_template.git
cd services_template
# check if the template is working
cargo generate --test
```

## Embedded Scripting

Learn more about Embedded Scripting for Rust [here](https://rhai.rs/book/about/index.html) , link for [LSP](https://github.com/rhaiscript/lsp) on vscode

Example usage of Rhai Scripting on the template [here](https://cargo-generate.github.io/cargo-generate/templates/scripting.mini-example.html#Rhai-extensions)


## [License](LICENSE)

## Pull Requests

If you need to make changes to the template, please submit a pull request.
