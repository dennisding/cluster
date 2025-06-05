
// mod network;
use cluster::network;

fn main() {
	network::service::say_hello();
	println!("Hello, world! cluster");
}
