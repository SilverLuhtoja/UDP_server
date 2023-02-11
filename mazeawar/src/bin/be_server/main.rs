use serde_json::json;
use std::collections::HashMap;
use std::net::SocketAddr;

mod server;
use crate::server::*;
pub use mazewar::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server= Server::new().await;
    let mut grid = Grid::new(10, 10, LOW);
    grid.generate_maze();
    let map = grid.convert_to_map();
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
            let player:Player = serde_json::from_value(data.data)?;
            *current_player = player;
        }

        if data.message_type == "shoot" {
            let shooter = players.get(&src).expect("ADD PLAYER < NOT IN HASH >");
            let mut cloned = players.clone();
            for (addr,player) in &players{
                if &src != addr && shooter.is_target_aligned(&player.clone()) {
                        if shooter.is_hit(&player, &map) {
                            let target =  cloned.get_mut(&addr).expect("ADD PLAYER < NOT IN HASH >");
                            target.location = map.get_spawn().await;
                    }
                }
            }
            players = cloned;
        }
        
        message.players = players.clone();
        for (addr,_) in &players{
            server.send_message(&message, addr).await
        }
    }
}
