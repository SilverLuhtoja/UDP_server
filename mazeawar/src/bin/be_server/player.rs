use mazewar::{Point, Direction, common::constants::BOX_SIZE};
use serde::{Deserialize, Serialize};

use crate::map::Map;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Player {
    pub location: Point,
    pub looking_at: Direction,
    pub username: String,
    pub score: i32,
}

impl Player {
    pub fn new(location: Point, name: String) -> Self {
        Self{
            location,
            looking_at: Direction::UP,
            username: name,
            score: 0,
        }        
    }

    pub fn is_target_aligned(&self, target: &Player) -> bool{
        self.location.x == target.location.x || self.location.y == target.location.y
    }

    pub fn is_hit(&self, target: &Player, map: &Map) -> bool{
        let target = (target.location.x / BOX_SIZE, target.location.y / BOX_SIZE); 
        let mut bullet = (self.location.x / BOX_SIZE,  self.location.y / BOX_SIZE);
        let difference = looking_direction_calculation_difference(self.looking_at);
        while bullet != target{
            if map.is_wall(bullet.1, bullet.0){
                return false
            }
            bullet = add_difference(bullet, difference)
        }
        true
    }
}

pub fn looking_direction_calculation_difference(face_dir: Direction) -> (f32,f32){
    match face_dir{
        Direction::UP => {(0.0,-1.0)},
        Direction::DOWN => {(0.0,1.0)},
        Direction::LEFT => {(-1.0,0.0)},
        Direction::RIGHT => {(1.0,0.0)},
    }
}

pub fn add_difference(x:(f32,f32), y:(f32,f32)) -> (f32,f32){
    (x.0+y.0,x.1+y.1)
}
