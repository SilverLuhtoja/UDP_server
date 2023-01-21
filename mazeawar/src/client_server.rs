use local_ip_address::local_ip;
use std::{net::{SocketAddr, UdpSocket, IpAddr, SocketAddrV4}, io};

use crate::player::Player;

// const ADDR: &str = "127.0.0.1";
const PORT: u16 = 34254;

// needs mutable hashmap for IP => {PlayerData}

#[derive(Debug)]
pub struct Client {
    pub socket: UdpSocket,
}


impl Client {
    pub fn new() -> Self {
        Self {
            socket: UdpSocket::bind(format!("{}", SocketAddrV4::new(std::net::Ipv4Addr::new(0, 0, 0, 0), 0))).unwrap(),
        }
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
