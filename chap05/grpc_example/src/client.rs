use grpc;
use grpc::ClientStubExt;

use crate::foobar::*;

pub fn run_client() {
	let client = FooBarServiceClient::new_plain(
		"127.0.0.1", 9001, grpc::ClientConf::new()).unwrap();

	let mut loc = Location::new();
	loc.latitude = 40.730610;
	loc.longitude = -73.935242;

	let mut req = CabLocationRequest::new();
	req.set_name("foo".to_string());
	req.set_location(loc);

	let ack = client.record_cab_location(
		grpc::RequestOptions::new(), req);

	match ack.wait() {
		Err(e) => panic!("{:?}", e),
		Ok((_, r, _)) => println!("{:?}", r),
	}

	//-------------------------------------------------

	let mut req = GetCabRequest::new();
	let mut loc = Location::new();
	loc.latitude = 40.730610;
	loc.longitude = -73.935242;
	req.set_location(loc);

	let ack = client.get_cabs(grpc::RequestOptions::new(), req);
	match ack.wait() {
		Err(e) => panic!("{:?}", e),
		Ok((_, r, _)) => println!("{:?}", r),
	}
}