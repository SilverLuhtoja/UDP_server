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

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero()->Self{
        Self { x: 0.0, y: 0.0 }
    }

    pub fn is_moved(&self)->bool{
        !(self.x == 0.0 && self.y == 0.0)
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Player{
    pub location: Point,
    pub looking_at:  Direction,
    pub username: String,
    pub score: i32,
}

impl Player {
    pub fn new(location: Point, username: String) -> Self {
        Self{
            location,
            looking_at: Direction::UP,
            username: username,
            score: 0,
        }        
    }
}
