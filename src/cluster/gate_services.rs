
use std::sync::mpsc;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::io::{self};

async fn serve_gate(mut socket: TcpStream) -> io::Result<()> {
	println!("start serve gate");

	loop {
		match process_packet(&mut socket).await {
			Err(e) => {
				println!("gate disconnected. {}", e);
				return Ok(());
			},
			Ok(()) => return Ok(()),
		}
	}
}

async fn process_packet(mut socket: &mut TcpStream) -> io::Result<()> {
	loop {
		let size = socket.read_u8().await?;
		println!("read package size: {}", size);

		let mut buffer: Vec<u8> = vec![0; size as usize];
		socket.read_exact(&mut buffer).await?;

		println!("package readed!");
		process_message(buffer);
	}

	Ok(())
}

fn process_message(buffer: Vec<u8>) -> io::Result<()>{
	println!("process message")
}

pub async fn serve_forever(port: u16, sender: mpsc::Sender<i32>) -> io::Result<()>{
	let address = format!("127.0.0.1:{}", port);
	let listener = TcpListener::bind(&address).await?;
	println!("gate serve at: {}", address);

	loop {
		let (mut socket, addr) = listener.accept().await?;
		println!("socket accepted: {}", &addr);

		tokio::spawn(serve_gate(socket));
	}
}
