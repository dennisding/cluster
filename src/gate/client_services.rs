
use crate::utils::uuid;
use std::collections::HashMap;
use tokio::net::tcp::OwnedReadHalf;
use tokio::sync::mpsc;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt};
use super::message::Message;

pub struct ClientServices {
    clients: HashMap<uuid::Uuid, String>,
}

impl ClientServices {
    pub fn new() -> Self {
        ClientServices {
            clients: HashMap::new(),
        }
    }

    pub async fn serve(sender: mpsc::Sender<Message>, port: i32) {
        let bind_addr = format!("127.0.0.1:{}", port);
        println!("client serve at: {}", bind_addr);
        let listener = TcpListener::bind(bind_addr).await.unwrap();

        let (stream, _) = listener.accept().await.unwrap();

        sender.send(Message::ClientServeReady).await.unwrap();

        let (reader, writer) = stream.into_split();

        tokio::spawn(async move {
            match Self::handle_client(&sender, reader).await {
                Ok(()) => (),
                _ => sender.send(Message::ClientDisconnected).await.unwrap(),
            }
        });
    }

    async fn handle_client(sender: &mpsc::Sender<Message>, mut reader: OwnedReadHalf) 
        -> Result<(), tokio::io::Error>
    {
        // handle client
        println!("handle client!");

        loop {
            let msg_id = reader.read_i32().await?;

            match msg_id {
                1 => Self::handle_message_hello(&sender, &mut reader).await?,
                _ => ()
            }
        }
    }

    async fn handle_message_hello(sender: &mpsc::Sender<Message>, reader: &mut OwnedReadHalf) 
        -> Result<(), tokio::io::Error>
    {
        let size = reader.read_i32().await?;
        let mut buffer = vec![0u8; size as usize];

        reader.read_exact(&mut buffer).await?;

        let msg = String::from_utf8(buffer).unwrap();
        sender.send(Message::ClientHello(msg)).await.unwrap();

        Ok(())
    }

}