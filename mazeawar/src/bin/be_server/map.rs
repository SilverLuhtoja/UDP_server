use mazewar::{map::map::{FLOOR, WALL}, Point, common::constants::BOX_SIZE};
use r::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
pub struct Map(
    pub Vec<Vec<i32>>,
);

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Self(vec![vec![FLOOR; width]; height])
    }

    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn is_wall(&self, row: f32, column: f32) -> bool{
        self.0[row as usize][column as usize] == 1
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

    pub fn remove_walls(&mut self, mut nb: i32) {
        while nb != 0 {
            let x = thread_rng().gen_range(1..(self.width() - 1));
            let y = thread_rng().gen_range(1..(self.height() - 1));
            if self.0[y][x] == WALL && (
                self.0[y - 1][x] == WALL && self.0[y + 1][x] == WALL && self.0[y][x - 1] == FLOOR && self.0[y][x + 1] == FLOOR ||
                    self.0[y][x - 1] == WALL && self.0[y][x + 1] == WALL && self.0[y - 1][x] == FLOOR && self.0[y + 1][x] == FLOOR
            ) {
                self.0[y][x] = FLOOR;
                nb -= 1;
            }
        }
    }
}
