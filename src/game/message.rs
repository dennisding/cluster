
//use crate::network::Packet;

pub enum Message {
    GameClientConnected,
    GameClientDisconnected,
    GameClientHello(String),
    // GamePacket(Packet),
    // GameError(String),
    // GatePacket(String),
    // Gateerror(String)
}

// pub struct GameMessage {
//     id: u8,
// }