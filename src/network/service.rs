

pub struct Service {
	connected : int,
}

pub impl Service {
	pub fn new() -> Service {
		return Service{
			connected : 10
		}
	}

	pub fn serve(&self) {

	}
}

fn serve<Type>(port : int, protocol : Type) {
	
}