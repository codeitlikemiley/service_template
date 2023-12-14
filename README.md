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

## Workflow

1. Create additional / new proto files in the `proto` directory.
Note: you can shove all your proto files in the `proto` directory.

2. Run `cargo build` to generate the GRPC service stubs.
after running `cargo build` you should see the following files generated:

```sh
src/<protobufname>.rs
```
it can be one or more files depending on the proto files you have in the `proto` directory.

3. Implement the GRPC service stubs in `src/<service_name>_impl.rs`.

- Add more methods to the GRPC service stubs.
- Add more crates to the `Cargo.toml` file as dependencies as needed.

4. Add on your workspace `Cargo.toml` the following:

```toml
[workspace]
resolver = "2"
members = [
  "server",
  "lib",
  # add here the new service
  "services/<service_name>
]
# if you download this repo and wanna use locally , you can exclude it on the workspace
# git clone codeitlikemiley/service_template template
# note you need to also add it on your .gitignore
exclude = ["template"]
```

5. Add more  tests and documentation on src/lib.rs

```rust
cargo test
cargo doc --open
```

For more info how to use Rust Tonic check this [link](https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md)


## [License](LICENSE)

## Pull Requests

If you need to make changes to the template, please submit a pull request.
