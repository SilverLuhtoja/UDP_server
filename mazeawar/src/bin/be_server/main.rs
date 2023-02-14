use game_logic::{is_map_change, reset_all, generate_new_map};
use mazewar::GameState;
use serde_json::json;
use std::collections::HashMap;
use std::net::SocketAddr;
use player::Player;

mod server;
mod map;
mod player;
mod maze;
mod game_logic;
use crate::server::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server= Server::new().await;
    let mut level = 1;
    let mut map = generate_new_map(level);
    let mut players:HashMap<SocketAddr, Player> = HashMap::new();
    
    let mut message = BroadcastMessage{
                map: json!(map),
                players : players.clone(),
                game_state: GameState::Game
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

        if data.message_type == "game on" {
            message.game_state = GameState::Game
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
                            target.location = map.get_spawn().await;
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
        
        if is_map_change(&players){
            level += 1;
            map = generate_new_map(level);
            message.map = json!(map);
            message.game_state = GameState::NewLevel;
            reset_all(&mut players, &map).await;
        }

        message.players = players.clone();
        for (addr,_) in &players{
            server.send_message(&message, addr).await
        }
    }
}
