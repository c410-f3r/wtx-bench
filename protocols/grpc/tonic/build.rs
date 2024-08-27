fn main() {
    tonic_build::compile_protos("./grpc.proto").unwrap();
}
