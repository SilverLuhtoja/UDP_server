use std::{net::{SocketAddr, UdpSocket, SocketAddrV4, Ipv4Addr}, collections::HashMap};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use serde_json::*;
use crate::{map::map::Map, player::player::Player, common::constants::BUFFER, GameState};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Message {
    message_type: String,
    data: JsonValue,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
pub struct Data{
    pub map: Map,
    pub players: HashMap<SocketAddr, Player>,
    pub game_state: GameState
}

#[derive(Debug)]
pub struct Client {
    pub socket: UdpSocket,
}

impl Client {
    pub fn new(server_addr: SocketAddr) -> Self {
        let  socket =  UdpSocket::bind(format!("{}", SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0))).expect("ERROR<connect>: bind to address failed");
        socket.connect(server_addr).expect("ERROR<connect>: failed to connect with main_server");
        Self {socket}
    } 

    pub fn send_data(&self, action: &str,  data: JsonValue){
        let message = Message {
            message_type : action.to_string(),
            data,
        };
        self.socket.send(json!(&message).to_string().as_bytes()).expect("ERROR<send>: failed to send a message");
    }
    
    pub fn send_action(&self, action: &str){
        let message = Message {
            message_type : action.to_string(),
            data: json!(""),
        };
        self.socket.send(json!(&message).to_string().as_bytes()).expect("ERROR<send>: failed to send a message");
    }

    pub fn read_message(&self) -> Data {
        let mut buf = [0; BUFFER];
        let (amt, _src) = self.socket.recv_from(&mut buf).expect("ERROR<read>: failed to receive message failed");
        let filled_buf = &mut buf[..amt];
        let incoming_message = String::from_utf8_lossy(filled_buf).into_owned();
        // println!("SERVER --> {:?}", incoming_message);
        serde_json::from_str(&incoming_message).expect("ERROR<read>: couldn't parse message")
    }

    pub fn get_address(&self) -> SocketAddr{
        self.socket.local_addr().expect("ERROR: Could not get local address")
    }
}
