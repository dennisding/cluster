
use tokio::sync::mpsc;
use tokio::net::tcp::OwnedReadHalf;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt};
use super::message::Message;

pub struct ClientServices {

}

impl ClientServices {
    pub fn new() -> Self {
        ClientServices {

        }
    }

    pub async fn serve(sender: mpsc::Sender<Message>, port: i32) -> Result<(), Box<dyn std::error::Error>> {
        let listen_addr = format!("127.0.0.1:{}", port);
        println!("listen address: {}", listen_addr);

        let accepter = TcpListener::bind(listen_addr).await?;

        loop {
            let (stream, _) = accepter.accept().await?;
            println!("accept: {}", stream.peer_addr().unwrap());
            let new_sender = sender.clone();

            let (reader, _) = stream.into_split();

            tokio::spawn(Self::handle_client_wrap(new_sender, reader));
        }
    }

    async fn handle_client_wrap(sender: mpsc::Sender<Message>, mut reader: OwnedReadHalf) {
        let result = Self::handle_client(&sender, &mut reader).await;
        match result {
            Ok(()) => (),
            Err(e) => {
                println!("error:{}", e)
            }
        }

        sender.send(Message::GameClientDisconnected).await.unwrap();
    }

    async fn handle_client(sender: &mpsc::Sender<Message>, reader: &mut OwnedReadHalf) 
        -> Result<(), std::io::Error>{
        // send client
        sender.send(Message::GameClientConnected).await.unwrap();
        loop {
            println!("read id:");
            let id = reader.read_i32().await?;
            println!("read id:{}", id);
            match id {
                1 => Self::handle_hello(&sender, reader).await?,
                _ => ()
            }
        }
    }

    async fn handle_hello(sender: &mpsc::Sender<Message>, reader: &mut OwnedReadHalf) 
        -> Result<(), std::io::Error> {

        let size = reader.read_i32().await?;
        let mut buffer = vec![0u8; size as usize];

        println!("handle hello : size {}", size);

        reader.read_exact(&mut buffer).await?;

        let msg = String::from_utf8(buffer).unwrap();

        sender.send(Message::GameClientHello(msg)).await.unwrap();
        Ok(())
    }

}