use glob::glob;

fn main() {
    let proto_files: Vec<_> = glob("proto/*.proto")
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .map(|path| path.to_string_lossy().into_owned())
        .collect();

    tonic_build::configure()
        .out_dir("src/")
        .compile(&proto_files, &["proto"])
        .unwrap_or_else(|e| panic!("Failed to compile protobuf {:?}", e));
}
