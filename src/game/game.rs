
use tokio::sync::mpsc;
use super::game_services::GameServices;
use super::client_services::ClientServices;
use super::config::Config;
use super::message::Message;

pub struct Game {
    game_services: GameServices,
    client_services : ClientServices,
}

impl Game {
    pub fn new() -> Self {
        Game {
            game_services: GameServices::new(),
            client_services: ClientServices::new(),
        }
    }

    pub fn serve_forever(&mut self) {
        let config = Config::new();
        let runtime = tokio::runtime::Runtime::new().unwrap();

        let (mut sender, mut receiver) = mpsc::channel::<Message>(config.channel_size);

        // client serve
        let client_serve = ClientServices::serve(sender.clone(), config.client_port);
        runtime.spawn(async {
            let _ = client_serve.await;
        });

        // gate serve
        loop {
            let result = receiver.try_recv();
            match result {
                Ok(Message::GameClientConnected) => self.on_game_client_connected(),
                Ok(Message::GameClientHello(msg)) => self.on_game_client_hello(msg),
                _ => {}
            }
        }
    }

    fn on_game_client_connected(&self) {
        println!("on game client connected");
    }

    fn on_game_client_hello(&self, msg: String) {
        println!("on game client hello: {}", msg);
    }

}
