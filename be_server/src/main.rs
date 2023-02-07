// #![allow(dead_code)]
// #![allow(unused_imports)]
mod map;
mod maze;
mod player;
mod server;

use crate::maze::{Grid, LOW};
use crate::server::*;
use player::{Player, Point};
use serde_json::json;
use std::collections::HashMap;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server= Server::new().await;
    let mut grid = Grid::new(10, 10, LOW);
    grid.generate_maze();
    let mut map = grid.convert_to_map();
    let mut players:HashMap<SocketAddr, Player> = HashMap::new();
    
    let mut message = BroadcastMessage{
                map: json!(map),
                players : players.clone()
            };
    
    loop {
        let (data, src) = server.read_message().await;
        
        if data.message_type == "connect" {
            println!("CONNECTING WITH  --> {}", src);
            let spawn = map.get_spawn().await;
            let player = Player::new(spawn);
            players.insert(src,player);
        }

        if data.message_type == "movement" {
            let current_player = players.get_mut(&src).expect("ADD PLAYER < NOT IN HASH >");
            map.remove_player(&current_player);
            let player:Player = serde_json::from_value(data.data)?;
            *current_player = player;
            map.set_player(&current_player)
        }
        
        message.map = json!(map.clone());
        message.players = players.clone();
        for (addr,_) in &players{
            server.send_message(&message, addr).await
        }
    }
}

