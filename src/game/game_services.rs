
use std::os::windows::io::AsSocket;

use super::message::Message;
use tokio::sync::mpsc;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct GameServices {

}

impl GameServices {
    pub fn new() -> Self {
        GameServices {

        }
    }

    pub async fn serve(sender: mpsc::Sender<Message>, port: i32) 
        -> Result<(), Box<dyn std::error::Error>> {
        
        let listen_addr = format!("127.0.0.1:{}", port);
        println!("listening:{}", listen_addr);

        let listener = TcpListener::bind(listen_addr).await?;

        loop {
            let (mut stream, _) = listener.accept().await?;
        }


        Ok(())
    }
}
