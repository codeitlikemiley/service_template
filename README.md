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
├── frontend
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

Note: The following command will generate a new GRPC service template.
```bash
cd <workspace_root>/services
cargo generate --git codeitlikemiley/service_template --name <service_name>
cd <service_name>
```

output:
```sh
cargo generate --git codeitlikemiley/service_template --name <service_name>
🔧   Destination: /Users/uriah/Code/example
🔧   project-name: example ...
🔧   Generating template ...
🤷   Enter a brief description of the service: [default: Default Service Description]: Default Service Description
🤷   Enter the RPC method name: [default: make]: make
🔧   Moving generated files into: `/Users/uriah/Code/example`...
🔧   Initializing a fresh Git repository
✨   Done! New project created /Users/uriah/Code/example
```

## Development

```bash
git clone https://github.com/codeitlikemiley/service_template.git
cd service_template
# check if the template is working
cargo generate --test
```

## [License](LICENSE)

## Pull Requests

If you need to make changes to the template, please submit a pull request.
