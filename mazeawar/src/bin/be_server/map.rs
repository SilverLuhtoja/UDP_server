use mazewar::{map::map::FLOOR, Point, common::constants::BOX_SIZE};
use r::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
pub struct Map(
    pub Vec<Vec<i32>>,
);

impl Map {
    pub fn new_from_arr(map: Vec<Vec<i32>>) -> Self{
        Self(map)
    }

    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub async fn get_spawn(&self) -> Point {
        loop {
            let row = thread_rng().gen_range(1..self.height() - 1);
            let column = thread_rng().gen_range(1..self.width() - 1);
            if self.0[row][column] == FLOOR {
                return Point::new(column as f32 * BOX_SIZE, row as f32 * BOX_SIZE);
            }
        }
    }

    pub fn is_wall(&self, row: f32, column: f32) -> bool{
        self.0[row as usize][column as usize] == 1
    }
}
