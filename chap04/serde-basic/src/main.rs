use serde;
use serde::Deserialize;
use serde::Serialize;
use serde_json;
use serde_yaml;

#[derive(Serialize, Deserialize, Debug)]
struct ServerConfig {
	workers: u64,
	ignore: bool,
	auth_server: Option<String>,
}

fn main() {
	let cfg = ServerConfig {
		workers: 100,
		ignore: false,
		auth_server: Some("auth.server.io".to_string()),
	};

	let strval = serde_yaml::to_string(&cfg).unwrap();
	println!("To YAML:\n{}\n", strval);
	let obj: ServerConfig = serde_yaml::from_str(strval.as_str()).unwrap();
	println!("From yaml str:\n{:?}\n\n", obj);

	let strval = serde_json::to_string(&cfg).unwrap();
	println!("To JSON:\n{}\n", strval);
	let obj: ServerConfig = serde_json::from_str(strval.as_str()).unwrap();
	println!("From json str:\n{:?}\n\n", obj);
}
