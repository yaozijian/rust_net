use protoc_rust_grpc;

fn main() {
	protoc_rust_grpc::run(protoc_rust_grpc::Args {
		out_dir: "src/foobar",
		includes: &[],
		input: &["foobar.proto"],
		rust_protobuf: true,
	}).expect("为proto文件生成Rust代码失败");
}