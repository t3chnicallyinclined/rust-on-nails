use std::io;

fn main() -> Result<(), io::Error> {

    tonic_build::configure()
    .compile(
        &["api.proto"], // Files in the path
        &["../grpc-api/protos"], // The path to search
    )
    .unwrap();
    println!("cargo:rerun-if-changed=api.proto");

    Ok(())
}