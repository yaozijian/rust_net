use std::thread;

use foobar::*;
use server::*;

mod foobar;
mod server;
mod client;

fn main() {
	let args = std::env::args().nth(1).unwrap();
	if args == "--server" {
		run_server();
	} else if args == "--client" {
		client::run_client();
	}
}

fn run_server() {
	let mut server = grpc::ServerBuilder::new_plain();
	server.http.set_port(9001);
	server.add_service(FooBarServiceServer::new_service_def(FooBarServer));
	server.http.set_cpu_pool_threads(4);
	let _s = server.build().expect("启动服务器失败");
	loop {
		thread::park();
	}
}
