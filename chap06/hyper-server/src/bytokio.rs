use std::net::SocketAddr;

use futures::future::Future;
use futures::Stream;
use hyper::server::conn::Http;
use hyper::service::service_fn;
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;

fn serve(addr: &SocketAddr) {
	let mut core = Core::new().unwrap();
	let handle = core.handle();
	let lis = ::std::net::TcpListener::bind(addr).unwrap();
	let lis = TcpListener::from_listener(lis, addr, &handle).unwrap();
	core.run(lis.incoming().for_each(|(sock, _addr)| {
		let c =
			Http::new().serve_connection(sock, service_fn(super::echo::echo));
		let fut = c.map_err(|e| eprintln!("服务器连接出错: {}", e));
		hyper::rt::spawn(fut);
		Ok(())
	})).unwrap();
}

pub fn by_tokio() {
	let s: SocketAddr = "127.0.0.1:3000".parse().unwrap();
	serve(&s);
}


