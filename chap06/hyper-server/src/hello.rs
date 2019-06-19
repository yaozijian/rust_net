use std::{thread, time};

use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

fn heavy_work() -> String {
	let dura = time::Duration::from_millis(200);
	thread::sleep(dura);
	"done".to_string()
}

fn hello_world(_req: Request<Body>) -> Response<Body> {
	Response::new(Body::from(heavy_work()))
}

pub fn hello() {
	let new_svc = || {
		service_fn_ok(hello_world)
	};

	let addr = ([127, 0, 0, 1], 3000).into();

	let server = Server::bind(&addr)
		.serve(new_svc)
		.map_err(|e| eprintln!("server error: {}", e));

	hyper::rt::run(server);
}

