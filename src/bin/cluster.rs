
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

// use cluster::cluster;
use cluster::cluster;

// mod network;
#[tokio::main]
async fn main() -> io::Result<()> {

	cluster::serve_forever(1000, 1001).await?;
	

	Ok(())
//	cluster::serve().await?;
	// let listener = TcpListener::bind("127.0.0.1:1234").await?;

	// loop {
	// 	let (mut socket, _) = listener.accept().await?;

	// 	tokio::spawn(async move {
	// 		let mut buffer = [0; 1024];
	// 		loop {
	// 			let n = match socket.read(&mut buffer).await {
	// 				Ok(0) => return,
	// 				Ok(n) => n,
	// 				Err(e) => {
	// 					println!("read error:{}", e);
	// 					return;
	// 				}
	// 			};
	// 			// write back
	// 			if let Err(e) = socket.write_all(&buffer[..n]).await {
	// 				println!("write erro: {}", e);
	// 				return;
	// 			}
	// 		}
	// 	});
	//}
}