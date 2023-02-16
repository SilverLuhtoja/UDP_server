use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use serde_json::*;
use local_ip_address::local_ip;
use tokio::net::UdpSocket;
use std::{net::{SocketAddr, Ipv4Addr, IpAddr}, collections::HashMap};

use mazewar::{common::constants::BUFFER, GameState};

use crate::player::Player;

const PORT: u16 = 4242;

#[derive(Clone, Debug, PartialEq,Deserialize, Serialize)]
pub struct Data{
   pub message_type: String,
   pub data: JsonValue,
}

#[derive(Clone, Debug, PartialEq,Deserialize, Serialize)]
pub struct BroadcastMessage{
   pub map: JsonValue,
   pub players : HashMap<SocketAddr, Player>,
   pub game_state: GameState
}

#[derive(Debug)]
pub struct Server {
    pub socket: UdpSocket,
}

impl Server {
    pub async fn new() -> Self {
        let my_local_ip = local_ip().unwrap();
        let socket = UdpSocket::bind(format!("{}:{}", my_local_ip, PORT)).await.expect("ERROR<connect>: bind to address failed");
        println!("Server running on: {}", socket.local_addr().unwrap());
        Self {socket}
    } 

    pub async fn send_message(&self, message: &BroadcastMessage, addr: &SocketAddr){
        self.socket.send_to(json!(&message).to_string().as_bytes(),&addr).await.expect("ERROR<send>: failed to send a message");
    }

    // pub async  fn read_message(&self) -> (Data, SocketAddr) {
    //     let mut buf = [0; BUFFER];
    //     let (amt, _src) = self.socket.recv_from(&mut buf).await.expect("ERROR<read>: failed to receive message failed");
    //     let incoming_message = String::from_utf8_lossy(&buf[..amt]).into_owned();
    //     let data: Data = serde_json::from_str(&incoming_message).expect("ERROR<read>: couldn't parse message");
    //     println!("Received message from <{}>: {:?}", _src, data.message_type);
    //     return (data, _src)
    // }

    pub async fn read_message(&self) -> Option<(Data, SocketAddr)> {
        let mut buf = [0; BUFFER];

        match self.socket.recv_from(&mut buf).await {
            Ok((amt, src)) => {
                let incoming_message = String::from_utf8_lossy(&buf[..amt]).into_owned();
                let data: Data = serde_json::from_str(&incoming_message).expect("ERROR<read>: couldn't parse message");
                Some((data, src))
            },
            Err(_) => {
                let error_data = Data{
                        message_type : String::from(""),
                        data : json!("")
                    };
                let error_addr =  SocketAddr::new(IpAddr::V4(Ipv4Addr::new(7,7, 7, 7)), 7777);
                Some((error_data,error_addr))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr};

    use super::*;
    
    #[tokio::test]
    async fn test_new() {
        let server = UdpSocket::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 4242)).await.unwrap();
        let server_local = server.local_addr().unwrap().to_string();
        assert_eq!(server_local, "0.0.0.0:4242");
    }
}
