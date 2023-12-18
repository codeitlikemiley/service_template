# GRPC Service Generator

[![Build Service Template](https://github.com/codeitlikemiley/service_template/actions/workflows/build.yml/badge.svg)](https://github.com/codeitlikemiley/service_template/actions/workflows/build.yml)

## Requirements

Note: Install the following before you can use the template generator.

- [rust, rustup, cargo](https://www.rust-lang.org/tools/install)
- [cargo-generate](https://cargo-generate.github.io/cargo-generate/installation.html)
- [protoc](https://grpc.io/docs/protoc-installation/)
- [server_template](https://github.com/codeitlikemiley/server_template)
- [workspacer](https://github.com/codeitlikemiley/workspacer) (optional cli tools to manage rust workspace)

## GRPC Client

- [grpcurl](https://github.com/fullstorydev/grpcurl)
- [evans_cli](https://github.com/ktr0731/evans)
- [Postman](https://www.postman.com/downloads/)


## Workspace Setup

> Note: You can ignore this step if you already set it up

- [ ] Set up Workspace
```sh
mkdir <workspace_root>
cd <workspace_root>
```

- [ ] Update workspace `Cargo.toml` file content below.
      
Note: The workspacer command above would generate this Code on `Cargo.toml`

```toml
[workspace]
resolver = "2"
members = [
    server,
    # add here services here after you generated it
    auth, # example service
]
```

or you can download and install [workspacer cli](https://github.com/codeitlikemiley/workspacer) and use it to manage your workspace

```sh
mkdir <workspace_root>
workspacer init
# by default it has the server in the member
# to add more services
worspacer add services/auth
# you can also remove service
workspacer rm service/auth
# to view all current members of workspace
workspacer ls
```

Example Workspace Tree Structure
```sh
workspace (root)
├── Cargo.toml
│
├── backend (server_template)
│
├── frontend (dioxus_template)
│
└── services (service_template)
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

- [ ] Create the Server

```sh
cd <workspace_root>
cargo generate --git codeitlikemiley/server_template --name server
```

## Favorite The template 

`~/.cargo/cargo-generate.toml` if this file dont exist please create it first `touch ~/.cargo/cargo-generate.toml`

- [ ] Create Folder for Templates

```sh
cd ~/.cargo
mkdir templates
```
- [ ] Add to favorites [services_template](https://github.com/codeitlikemiley/services_template) and [server_template](https://github.com/codeitlikemiley/server_template)

```toml
[values]
gh_username = "YOUR_USERNAME"
ide = "none|vscode"

[favorites.services]
path = "/Users/uriah/.cargo/templates/services_template"

[favorites.server]
path = "/Users/uriah/.cargo/templates/server_template/"
```

- [ ]  Generate Server and Services template with short cut

```sh
cargo generate services --name auth
cargo generate server --name server
```


## Usage: Generating a new GRPC Service from the template

- [ ] Change directory to the services directory.

```bash
cd <workspace_root>/services
```

- [ ] Generate a new GRPC Service.

```sh
cargo generate --git codeitlikemiley/services_template --name auth

```

### Test your Service with GRPC Client

1. Run Server

```sh
cargo run -p server
```

2. Use `grpcurl` Client

To invoke the specific rpc method use the following command
```sh
grpcurl -plaintext -import-path ./services/auth/proto -proto auth.proto -d '{"name": "Tonic"}' '[::1]:50051' auth.AuthService.Login
```

example output:
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

## [License](LICENSE)

## Pull Requests

If you need to make changes to the template, please submit a pull request.

## Learning Resources

For more info how to use cargo-generate check this [link](https://cargo-generate.github.io/cargo-generate/usage.html)

Learn about Liquid Templating [here](https://shopify.github.io/liquid/)

To learn more about Rust Tonic click this [link](https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md)

Learn more about Embedded Scripting for Rust [here](https://rhai.rs/book/about/index.html) , link for [LSP](https://github.com/rhaiscript/lsp) on vscode

Example usage of Rhai Scripting on the template [here](https://cargo-generate.github.io/cargo-generate/templates/scripting.mini-example.html#Rhai-extensions)
