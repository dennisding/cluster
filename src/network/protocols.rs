

pub enum GameProtocols {
    GameConnected,
    GameClientHello(i32, String),
}


pub struct Hello {
    id : i32,
    msg : String,
}

impl Hello {
}