
mod network;

struct Cluster{
	connected : int,
};

impl Cluster{
	pub fn new() -> Cluster {

	}
}

fn main() {
	network::service::say_hello();
	println!("Hello, world! cluster");
}
