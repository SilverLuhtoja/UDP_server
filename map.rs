use macroquad::color::{BLACK, RED, WHITE};
use macroquad::shapes::draw_rectangle;

pub const FLOOR: i32 = 0;
pub const WALL: i32 = 1;

#[derive(Clone, Debug, PartialEq)]
pub struct Map(pub Vec<Vec<i32>>);

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
    pub fn draw(&self) {
        let offset: f32 = 300.0;
        let size: f32 = 10.0;
        for i in 0..self.height() {
            for j in 0..self.width() {
                if self.0[i][j] == WALL {
                    draw_rectangle(j as f32 * size + offset, i as f32 * size + offset, size, size, WHITE);
                } else {
                    draw_rectangle(j as f32 * size + offset, i as f32 * size + offset, size, size, BLACK);
                }
            }
        }
    }
}
