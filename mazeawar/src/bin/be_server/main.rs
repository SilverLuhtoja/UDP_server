use maze::{Grid, LOW};
use mazewar::player::shoot;
use serde_json::json;
use std::collections::HashMap;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use map::Map;
use player::Player;

mod server;
mod map;
mod player;
mod maze;
use crate::server::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server= Server::new().await;
    let mut grid = Grid::new(10, 10, LOW);
    grid.generate_maze();
    let map = grid.convert_to_map();
    let mut players:HashMap<SocketAddr, Player> = HashMap::new();
    

    // let map_layout:Vec<Vec<i32>> = vec![
    // vec![1, 1, 1, 1, 1, 1],
    // vec![1, 0, 0, 0, 0, 1],
    // vec![1, 0, 0, 0, 0, 1],
    // vec![1, 0, 1, 1, 0, 1],
    // vec![1, 0, 0, 0, 0, 1],
    // vec![1, 0, 0, 0, 0, 1],
    // vec![1, 1, 1, 1, 1, 1]
    // ];
    // let mut map = Map::new_from_arr(map_layout);
    // let decoy:Player = Player::new(map.get_spawn().await, String::from("Miki")) ;
    // let decoy_addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 8, 102)), 0);
    // players.insert(decoy_addr,decoy);


    let mut message = BroadcastMessage{
                map: json!(map),
                players : players.clone()
            };
    
    loop {
        let (data, src) = server.read_message().await;
        
        if data.message_type == "connect" {
            println!("CONNECTING WITH  --> {}", src);
            let spawn = map.get_spawn().await;
            let username = &data.data;
            let player = Player::new(spawn, username.to_string());
            players.insert(src,player);
            println!("LIST: {:?}", players);
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
                if &src != addr && shooter.is_target_aligned(&player) {
                        if shooter.is_hit(&player, &map) && player.alive{
                            let target =  cloned.get_mut(&addr).expect("ADD PLAYER < NOT IN HASH >");
                            target.alive = false;
                            let shooter = cloned.get_mut(&src).expect("ADD PLAYER < NOT IN HASH >");
                            shooter.score += 1;
                    }
                }
            }
         
            players = cloned;
        }

        if data.message_type == "revive" {
            let mut current_player = players.get_mut(&src).expect("ADD PLAYER < NOT IN HASH >");
            current_player.alive = true;
            current_player.location = map.get_spawn().await;
        }
        
        message.players = players.clone();
        for (addr,_) in &players{
            // if addr != &decoy_addr{
            //     server.send_message(&message, addr).await
            // }
            server.send_message(&message, addr).await
        }
    }
}
