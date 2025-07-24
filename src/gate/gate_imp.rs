

use tokio;
use tokio::sync::mpsc;
use super::message::Message;
use super::config::Config;
use super::client_services::ClientServices;

pub struct Gate {
    client_services: ClientServices,
}

impl Gate {
    pub fn new() -> Self {
        Gate {
            client_services: ClientServices::new(),
        }
    }

    pub fn serve_forever(&mut self) {
        let config = Config::new();

        let runtime = tokio::runtime::Runtime::new().unwrap();
        let (sender, receiver) = mpsc::channel::<Message>(config.channel_size);

        runtime.spawn(ClientServices::serve(sender.clone(), config.client_port));
    }
}