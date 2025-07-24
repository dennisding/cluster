

use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt};

fn main() {
    println!("Hello world! client!!!");
    let runtime = tokio::runtime::Runtime::new().unwrap();

    runtime.spawn(async {
        let mut stream = TcpStream::connect("127.0.0.1:3000").await.unwrap();

        let msg = String::from("a message from client!!!!");
        let _ = stream.write_i32(1).await;
        let _ = stream.write_i32(msg.len() as i32).await;
        let _ = stream.write_all(msg.as_bytes()).await;

        loop {}
    });

    loop {}
}

