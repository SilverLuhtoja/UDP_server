use macroquad::prelude::*;


#[derive(Clone,Copy, Debug, PartialEq)]
pub enum  Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(Clone,Copy, Debug, PartialEq)]
pub struct Point{
    pub x: f32,
    pub y: f32
}

#[derive(Clone,Copy, Debug, PartialEq)]
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

impl Player {
    pub fn new(location: Point) -> Self {
        Self{
            location,
            looking_at: Direction::UP
        }        
    }

    pub fn update_position_direction(&mut self, next_position: Point){
        self.location = next_position;
    }

    pub fn draw(&self){
        // draw_rectangle(screen_width() / 1.25 , screen_height() / 2.0,20.0, 20.0, GREEN);
        draw_rectangle(self.location.x , self.location.y, 20.0,  20.0,  GREEN);
    }
}
