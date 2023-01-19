use std::{net::{UdpSocket, IpAddr}};

const PORT: u16 = 34254;
#[derive(Debug)]
pub struct Client {
    pub socket: UdpSocket,
}

impl Client {
    pub fn new(IP: IpAddr) -> Self {
        Self {
            socket: UdpSocket::bind(format!("{}:{}", IP, PORT)).unwrap(),
        }
    }
}
