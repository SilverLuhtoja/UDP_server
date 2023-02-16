use mazewar::SocketAddr;
use std::collections::HashMap;

use crate::{map::Map,maze::{Grid, HIGH, LOW, MEDIUM},player::Player};

pub fn zero_all_hearts(hearts:&mut HashMap<SocketAddr, usize>){
    for beats in hearts.values_mut(){
        *beats = 0;
    }
}

pub fn is_map_change(players: &HashMap<SocketAddr, Player>) -> bool {
    for player in players.values() {
        if player.score == 3 {
            return true;
        }
    }
    false
}

pub async fn reset_all(players: &mut HashMap<SocketAddr, Player>, map: &Map) {
    for src in players.to_owned().keys() {
        let mut player = players.get_mut(&src).expect("ADD PLAYER < NOT IN HASH >");
        player.location = map.get_spawn().await;
        player.score = 0;
        player.alive = true;
    }
}

pub fn generate_new_map(level: i32) -> Map {
    let difficulty = set_map_diffculty(level);
    let mut grid = Grid::new(10, 10, difficulty);
    grid.generate_maze();
    grid.convert_to_map()
}

fn set_map_diffculty(val: i32) -> i32 {
    return match val {
        1 => LOW,
        2 => MEDIUM,
        _ => HIGH,
    };
}
