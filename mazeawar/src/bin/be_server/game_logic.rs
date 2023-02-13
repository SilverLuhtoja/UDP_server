use mazewar::SocketAddr;
use std::collections::HashMap;

use crate::{player::Player, map::Map};

pub fn is_map_change(players: &HashMap<SocketAddr, Player>) -> bool{
    for player in players.values(){
        if player.score == 3 {
            return true
        }
    }
    false
}

pub async fn reset_all(players: &mut HashMap<SocketAddr, Player>,map: &Map){
    for src in players.to_owned().keys(){
       let mut player = players.get_mut(&src).expect("ADD PLAYER < NOT IN HASH >");
       player.location = map.get_spawn().await;
       player.score = 0;
       // player.alive = true;
    }
}
