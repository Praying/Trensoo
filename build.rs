extern crate protoc_grpcio;

fn main() {
    let proto_root = "protos";
    let proto_out="src/protos";
    println!("cargo:rerun-if-changed={}", proto_root);
    protoc_grpcio::compile_grpc_protos(&["helloworld.proto","diner.proto"],
                                       &[proto_root],
                                       &proto_out, None)
        .expect("Failed to compile gRPC definitions!");
}
