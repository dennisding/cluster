
mod network;

use network::protocols::Protocols;
use network::packet::Packet;
use network::services::Services;

struct ClientServices {

}

impl ClientServices {

    pub fn handle_hello(&mut self, mut packet: Packet) {
        println!("handle hello!!!");
        let msg = packet.read_string();
        if let None = msg {
            println!("invalid hello msg");
            return
        }

        println!("hello msg: {}", msg.unwrap());
    }

    pub fn handle_login(&mut self, mut packet: Packet) {
        println!("handle login");
        let name = packet.read_string();
        if let None = name {
            println!("invalid login name");
            return;
        }

        let password = packet.read_string();
        if let None = name {
            println!("invlaid login password");
            return;
        }

        println!("login: {}: {}", name.unwrap(), password.unwrap());
    }
}

impl Protocols for ClientServices {
    fn on_ready(&mut self) {
        println!("clinet services ready!!!");
    }

    fn on_packet(&mut self, mut packet: Packet) {
        let pid = packet.read_i32();

        if let None = pid {
            println!("invlaid pid");
            return;
        }

        match pid.unwrap() {
            1 => self.handle_hello(packet),
            2 => self.handle_login(packet),
            _ => {}
        }
    }
}

fn main() {
    println!("hello world");
    let client_services = ClientServices{};

    let mut service = Services::new(Box::new(client_services));

    service.serve_forever(1024);
}
