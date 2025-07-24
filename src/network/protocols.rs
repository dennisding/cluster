

use super::packet::Packet;

pub trait Protocols {
    fn on_ready(&mut self);
    fn on_packet(&mut self, packet: Packet);
}