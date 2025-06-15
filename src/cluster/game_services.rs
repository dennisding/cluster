
use std::sync::mpsc;

pub async fn serve_forever(port: u16, sender: mpsc::Sender<i32>) {
	println!("game service serve: {}", port);
}