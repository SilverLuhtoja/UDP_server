use serde::{Deserialize, Serialize};

#[derive(Clone,Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Point{
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn is_moved(&self) -> bool {
        !(self.x == 0.0 && self.y == 0.0)
    }
}
