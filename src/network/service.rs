

pub struct Service {
	pub connected : i32,
}

impl Service {
	pub fn new() -> Service {
		return Service{
			connected : 10
		}
	}

	pub fn serve(&self) {

	}
}

pub fn say_hello() {
	println!("hello");
}
