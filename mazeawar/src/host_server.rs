use local_ip_address::local_ip;
use std::net::{SocketAddr, UdpSocket, IpAddr};

use crate::player::Player;

// const ADDR: &str = "127.0.0.1";
const PORT: u16 = 4242;

// needs mutable hashmap for IP => {PlayerData}

#[derive(Debug)]
pub struct Host {
    pub socket: UdpSocket,
    pub clients: Vec<SocketAddr>,
    pub clients_data: Vec<Player>
}


impl Host {
    pub fn new(IP: IpAddr) -> Self {
        Self {
            socket: UdpSocket::bind(format!("{}:{}", IP, PORT)).unwrap(),
            clients: vec![],
            clients_data : vec![]
        }
    }

    pub fn add_player(&mut self, player: Player){
        self.clients_data.push(player)
    }
    pub fn listen_events() {}

    pub fn broadcast_to_players() {}

    pub fn read_incoming_messages(&self) {
        let mut buf = [0; 2048];
        let (amt, _src) = self.socket.recv_from(&mut buf).expect("incoming message failed");
        let filled_buf = &mut buf[..amt];
        let incoming_message = String::from_utf8_lossy(filled_buf).into_owned();
        println!("SERVER --> {:?}", incoming_message);
    }

}

// fn main() -> std::io::Result<()> {
//     // for UDP4
//     // let socket = UdpSocket::bind("[::]:2000")?;  // for UDP4/6
//     let socket = UdpSocket::bind(format!("{}:{}",ADDR, PORT))?;
//     let mut buf = [0; 2048];

//     println!("Creating server : {:?}.Listening....", socket);
//     println!("Waiting messages:");
//     loop {
//         println!();
//         // (bite_slice, address where it came from)
//         let (amt, src) = socket.recv_from(&mut buf).expect("incoming message failed");
//         let incoming_message = String::from_utf8_lossy(&mut buf[..amt]);
//         println!("client <{}>: {:?}", src, incoming_message);

//         if incoming_message == "connect" {
//             let message  = format!("Successful connection with {}:{}",ADDR,PORT);
//             socket.send_to(message.as_bytes(), &src)?;
//         }else{
//             // Redeclare `buf` as slice of the received data
//             // and send data back to origin.
//             let buf = &mut buf[..amt];
//             socket.send_to(&buf, &src)?;
//         }
//     }
// }
