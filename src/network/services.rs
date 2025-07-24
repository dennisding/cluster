
use tokio;
use tokio::net::{TcpListener, TcpStream};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;

use super::protocols::Protocols;
use super::packet::Packet;

enum Message {
    ServicesReady,
    OnConnected,
    Packet(Packet)
}

pub struct Services {
    pub protocols: Box<dyn Protocols>
}

impl Services {
    pub fn new(protocols: Box::<dyn Protocols>) -> Self {
        Services {
            protocols,
        }
    }

    pub fn serve_forever(&mut self, port: i32) {
        let runtime = tokio::runtime::Runtime::new().unwrap();

        let (sender, mut receiver) = mpsc::channel::<Message>(100);

        runtime.spawn(Self::serve_at(sender.clone(), port));

        loop {
            let msg = receiver.blocking_recv().unwrap();
            match msg {
                Message::ServicesReady => self.protocols.on_ready(),
                Message::OnConnected => {},
                Message::Packet(packet) => self.protocols.on_packet(packet),
            }
        }
    }

    pub async fn serve(&mut self, port: i32) {

    }

    async fn serve_at(sender: mpsc::Sender<Message>, port: i32) {
        let address = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(address.clone()).await.unwrap();

        sender.send(Message::ServicesReady).await.unwrap();

        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let (reader, writer) = stream.into_split();

            sender.send(Message::OnConnected).await.unwrap();
            tokio::spawn(Self::handle_connection(sender.clone(), reader));
        }
    }

    async fn handle_connection(sender: mpsc::Sender<Message>, mut reader: OwnedReadHalf) {
        let size_result = reader.read_i32().await;
        if let Ok(size) = size_result {
            let mut buffer = vec![0u8; size as usize];

            let result = reader.read_exact(&mut buffer).await;
            if let Err(e) = result {
                println!("error in read packet data: {}", e);
                return;
            }

            sender.send(Message::Packet(Packet::new(buffer))).await.unwrap();
        }
        else {
            println!("error in reading packet size");
        }
    }
}