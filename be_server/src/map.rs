use crate::player::{Point, Player};
use r::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

pub const WALL: i32 = 1;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Map(pub Vec<Vec<i32>>);

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Self(vec![vec![0; width]; height])
    }
    pub fn new_from_arr(map: Vec<Vec<i32>>) -> Self{
        Self(map)
    }

    pub fn set_player(&mut self, player: &Player){
        let column = (player.location.x / 20.0) as usize;
        let row = (player.location.y / 20.0 ) as usize;
        self.0[row][column] = 2;
    }

    pub fn remove_player(&mut self, player: &Player){
        let column = (player.location.x / 20.0) as usize;
        let row = (player.location.y / 20.0 ) as usize;
        self.0[row][column] = 0;
    }

    pub async fn get_spawn(&self) -> Point {
        loop {
            let row = thread_rng().gen_range(1..self.0.len() - 1);
            let column = thread_rng().gen_range(1..self.0.len() - 1);
            if self.0[row][column] == 0 {
                return Point::new(row as f32 * 20.0, column as f32 * 20.0);
            }
        }
    }
}
