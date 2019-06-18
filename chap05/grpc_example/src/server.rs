use crate::foobar::*;

pub struct FooBarServer;

impl FooBarService for FooBarServer {
	fn record_cab_location(&self, _m: grpc::RequestOptions, req: CabLocationRequest) -> grpc::SingleResponse<CabLocationResponse> {
		let mut r = CabLocationResponse::new();
		println!("Recorded cab {} at {},{}",
		         req.get_name(),
		         req.get_location().latitude,
		         req.get_location().longitude,
		);

		r.set_accepted(true);

		grpc::SingleResponse::completed(r)
	}

	fn get_cabs(&self, _m: grpc::RequestOptions, _r: GetCabRequest) -> grpc::SingleResponse<GetCabResponse> {
		let mut r = GetCabResponse::new();
		let mut location = Location::new();
		location.latitude = 40.7128;
		location.longitude = -74.0060;

		let mut one = Cab::new();
		one.set_name("Limo".to_owned());
		one.set_location(location.clone());

		let mut two = Cab::new();
		two.set_name("Merc".to_owned());
		two.set_location(location.clone());

		let vec = vec![one, two];
		let cabs = ::protobuf::RepeatedField::from_vec(vec);

		r.set_cabs(cabs);

		grpc::SingleResponse::completed(r)
	}
}


