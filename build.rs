extern crate prost_build;

fn main() {
    prost_build::compile_protos(&["src/sdk/proto/abi.proto"], &["src/sdk/proto/"]).unwrap();
}
