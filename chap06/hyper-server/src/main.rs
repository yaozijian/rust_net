mod hello;
mod echo;
mod bytokio;

fn main() {
	let arg = std::env::args().nth(1).unwrap_or("hello".to_string());
	if arg == "hello" {
		hello::hello();
	} else if arg == "echo" {
		echo::echo_demo();
	} else if arg == "tokio" {
		bytokio::by_tokio();
	}
}

