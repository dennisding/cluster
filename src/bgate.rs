
mod gate;
mod network;
mod utils;

/// impl ClientProtocols {
///     #[protocols(1)]
///     pub fn hello_message(&mut self, cid: Uuid, msg: String) {
///         let packet = self.packer().login("name1", "password1");
///         self.services.send(cid, packet)
///     }
/// 
///     #[protocols(2)]
///     pub fn login(&mut self, cid, name: String, password: String) {
///         
///     }
/// }
/// macro_rules! vec {
///     let mut vec_tmp = Vec::new();
///     $(($name: expr), *) => {
///         $(
///             vec_tmp.push($name);
///         )*
///         vec_tmp
///     };
/// }
///  let protocols = network::Protocols::new()
///  protocols.add(1, HelloProtocol{})
///  protocols.add(2, Login{})

/// let protocols = ClientProtocols::new();
/// let services = network::services::Services::new(protocols);
/// let services.serve_forever();
/// 
/// let packet = protocols.packer().hello_message(string)
/// services.send_packet(cid, packet)
/// services.loop();

fn main() {
    let mut gate = gate::Gate::new();

    gate.serve_forever();
}