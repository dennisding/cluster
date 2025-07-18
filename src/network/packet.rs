

pub struct Packet {
    pub msg: String,
}

impl Packet {
    pub fn new() -> Self {
        Packet {
            msg: String::from("hello"),
        }
    }
}