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

const BOX_SIZE:f32 = 20.0;

impl Player {
    pub fn new(location: Point) -> Self {
        Self{
            location,
            looking_at: Direction::UP
        }        
    }

    pub fn draw(&self){
        draw_rectangle(self.location.x , self.location.y, BOX_SIZE,  BOX_SIZE,  GREEN);
        self.draw_facing_indicator()
    }

    pub fn draw_facing_indicator(&self){
        draw_rectangle(self.location.x , self.location.y, BOX_SIZE,  BOX_SIZE,  GREEN);
        let middle_offset:f32 = 7.5;
        match self.looking_at {
            Direction::UP => draw_rectangle(self.location.x + middle_offset ,self.get_center_y() - BOX_SIZE, 5.0,  5.0,  RED),
            Direction::DOWN => draw_rectangle(self.location.x + middle_offset , self.get_center_y() + BOX_SIZE, 5.0,  5.0,  RED),
            Direction::LEFT => draw_rectangle(self.get_center_x() - BOX_SIZE , self.location.y + middle_offset, 5.0,  5.0,  RED),
            Direction::RIGHT => draw_rectangle(self.get_center_x() + BOX_SIZE , self.location.y + middle_offset, 5.0,  5.0,  RED)
        }
    }

    pub fn set_postion(&mut self, point: Point){
        self.location = point;
    }
    pub fn get_center_x(&self) -> f32{
        self.location.x + BOX_SIZE/2.0
    }
    pub fn get_center_y(&self) -> f32{
        self.location.y + BOX_SIZE/2.0
    }
}
