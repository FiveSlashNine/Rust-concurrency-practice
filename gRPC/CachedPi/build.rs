fn main() {
    tonic_build::compile_protos("proto/calcpi.proto").unwrap();
}
