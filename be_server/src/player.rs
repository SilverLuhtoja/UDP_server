use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone,Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum  Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(Clone,Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Point{
    pub x: f32,
    pub y: f32
}

#[derive(Clone,Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Player{
    pub location: Point,
    pub looking_at:  Direction
}

impl Player {
    pub fn new(location: Point) -> Self {
        Self{
            location,
            looking_at: Direction::UP
        }        
    }
}
