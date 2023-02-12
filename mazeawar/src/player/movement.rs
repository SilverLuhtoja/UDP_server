use crate::{common::constants::BOX_SIZE, map::map::Map, utils::point::Point};
use super::player::{Player, Direction};

impl Player {
    pub fn make_move(&mut self, step: (f32, f32), map: &Map, enemy_positions: &Vec<Point>) -> bool {
        let new_point = add_difference((self.location.x,self.location.y), step);
        
        for point in enemy_positions {
            if point.x == new_point.0 && point.y == new_point.1 {
                return false;
            }
        }
        
        if can_step(new_point, map){
            self.location = Point::new(new_point.0,new_point.1);
            return true
        }
        false
    }
    
    pub fn step_difference(&self)  -> (f32,f32){
        match self.looking_at{
            Direction::UP => {(0.0,-BOX_SIZE)},
            Direction::DOWN => {(0.0,BOX_SIZE)},
            Direction::LEFT => {(-BOX_SIZE,0.0)},
            Direction::RIGHT => {(BOX_SIZE,0.0)},
        }
    }

    pub fn turn_left(&mut self) {
        match self.looking_at {
            Direction::UP => self.looking_at = Direction::LEFT,
            Direction::DOWN => self.looking_at = Direction::RIGHT,
            Direction::LEFT => self.looking_at = Direction::DOWN,
            Direction::RIGHT => self.looking_at = Direction::UP,
        }
    }

    pub fn turn_right(&mut self) {
        match self.looking_at {
            Direction::UP => self.looking_at = Direction::RIGHT,
            Direction::DOWN => self.looking_at = Direction::LEFT,
            Direction::LEFT => self.looking_at = Direction::UP,
            Direction::RIGHT => self.looking_at = Direction::DOWN,
        }
    }
}

pub fn can_step(new_location: (f32,f32), map: &Map) -> bool {
    let x = new_location.0 / BOX_SIZE;
    let y = new_location.1 / BOX_SIZE;
    return map.0[y as usize][x as usize] == 0;
}

pub fn add_difference(x:(f32,f32), y:(f32,f32)) -> (f32,f32){
    (x.0+y.0,x.1+y.1)
}

pub fn reverse_difference(x:(f32,f32)) -> (f32,f32){
    (x.0 * -1.0, x.1 * -1.0)
}
