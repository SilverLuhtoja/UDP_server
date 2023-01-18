use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

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
        Self{
            x,
            y
        }
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

    // TODO -REFACTOR (its doing 2 things and taking 3 parameters)
    // take Point and seperate setting Direction
    pub fn update_position_direction(&mut self, next_position_x: f32, next_position_y: f32, dir: Direction){
        if self.looking_at == dir{
            self.location.x = next_position_x;
            self.location.y = next_position_y;
        }
        self.looking_at = dir;
    }
    
    pub fn update_movement(&mut self){
         if is_key_pressed(KeyCode::A) { 
            self.update_position_direction(self.location.x - BOX_SIZE, self.location.y, Direction::LEFT)
        }
        if is_key_pressed(KeyCode::D) { 
            self.update_position_direction(self.location.x + BOX_SIZE, self.location.y,Direction::RIGHT)
        }
        if is_key_pressed(KeyCode::W) { 
            self.update_position_direction(self.location.x , self.location.y - BOX_SIZE,Direction::UP)
        }
        if is_key_pressed(KeyCode::S) { 
            self.update_position_direction(self.location.x , self.location.y + BOX_SIZE,Direction::DOWN)
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
