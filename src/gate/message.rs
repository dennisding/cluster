

pub enum Message {
    // 
    ClientServeReady,

    ClientConnected,
    ClientDisconnected,
    ClientHello(String),
}