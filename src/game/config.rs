
pub struct Config {
    pub game_port: i32,
    pub gate_port: i32,
    pub client_port: i32,
    pub channel_size: usize
}

impl Config {
    pub fn new() -> Self {
        Config {
            game_port: 1000,
            gate_port: 2000,
            client_port: 3000,
            channel_size: 1024,
        }
    }
}
