
use std::sync::mpsc;
use tokio::join;
use tokio::io;

pub mod gate_services;
pub mod game_services;

pub async fn serve_forever(gate_port: u16, game_port: u16) -> io::Result<()>{

	let (sender, receiver) = mpsc::channel::<i32>();

	let gate_serve = gate_services::serve_forever(gate_port, sender.clone());
	let game_serve = game_services::serve_forever(game_port, sender.clone());

	join!(gate_serve, game_serve);

	Ok(())
}