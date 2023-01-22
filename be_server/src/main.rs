#![allow(dead_code)]
#![allow(unused_imports)]

mod map;
mod maze;
mod player;

use local_ip_address::local_ip;
use player::{Player, Point};
use serde::{Deserialize, Serialize};
use serde_json::{json};
use serde_json::Value as JsonValue;
use crate::{maze::{Grid, LOW}};
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::net::UdpSocket;


const PORT: u16 = 4242;


#[derive(Clone, Debug, PartialEq,Deserialize, Serialize)]
struct Data{
    message_type: String,
    data: JsonValue,
}

#[derive(Clone, Debug, PartialEq,Deserialize, Serialize)]
struct BroadcastMessage{
    map: JsonValue,
    players : HashMap<SocketAddr, Player>
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let my_local_ip = local_ip().unwrap();
    let socket = UdpSocket::bind(format!("{}:{}", my_local_ip, PORT)).await?;
    println!("Server running on: {}", socket.local_addr()?);
    let mut buf = [0; 6000];


    let mut grid = Grid::new(10, 10, LOW);
    grid.generate_maze();
    let map = grid.convert_to_map();
    let mut players:HashMap<SocketAddr, Player> = HashMap::new();
    let mut message = BroadcastMessage{
                map: json!(map),
                players : players.clone()
            };
    
    loop {
        let (recv_len, src) = socket.recv_from(&mut buf).await?;
        let incoming_message = String::from_utf8_lossy(&buf[..recv_len]);
        let data: Data = serde_json::from_str(&incoming_message)?;
        println!("Received message from {}: {:?}", src, data);
        
        if data.message_type == "connect" {
            println!("CONNECTING {}", src);
            let multiplier = (players.len() + 1) as f32;
            let location = Point { x: 100.0 * multiplier, y: 100.0 * multiplier};
            let player = Player::new(location);
            players.insert(src,player);
        }
        // if data.message_type == "update" {
        //     let current_player = players.get_mut(&src).expect("ADD PLAYER < NOT IN HASH >");
        //     current_player.location = Point::new(current_player.location.x + 1.0, current_player.location.y + 1.0);
            
        // }
        
        if data.message_type == "movement" {
            let current_player = players.get_mut(&src).expect("ADD PLAYER < NOT IN HASH >");
            let point:Point = serde_json::from_value(data.data)?;
            current_player.location = point;
        }
        
        message.players = players.clone();
        for (addr,_) in &players{
            socket.send_to(json!(&message).to_string().as_bytes(), addr).await?;
        }
    }
}
