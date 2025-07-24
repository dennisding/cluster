

pub struct Config {
    pub channel_size: usize,
    pub client_port: i32,
}

impl Config {
    pub fn new() -> Self {
        Config {
            channel_size: 20,
            client_port: 1000,
        }
    }
}