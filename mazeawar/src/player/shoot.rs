use super::player::{Direction, Player};
use crate::{common::constants::BOX_SIZE, map::map::WALL, Map};
use macroquad::{prelude::VIOLET, shapes::draw_line};
use math::round;

impl Player {
    pub fn shoot(&self, map: Vec<Vec<i32>>) {
        let (x, y) = self.get_tiles();
        let mut final_point_x: f32 = self.get_center_x();
        let mut final_point_y: f32 = self.get_center_y();
        match self.looking_at {
            Direction::UP => {
                for i in 0..=y as usize {
                    if map[i][x as usize] == WALL {
                        final_point_y = i as f32 * BOX_SIZE + BOX_SIZE;
                    }
                }
            }
            Direction::DOWN => {
                for i in y as usize..map.len() {
                    if map[i][x as usize] == WALL {
                        final_point_y = i as f32 * BOX_SIZE;
                        break;
                    }
                }
            }
            Direction::LEFT => {
                for i in 0..=x as usize {
                    if map[y as usize][i] == WALL {
                        final_point_x = i as f32 * BOX_SIZE + BOX_SIZE;
                    }
                }
            }
            Direction::RIGHT => {
                for i in x as usize..map[0].len() {
                    if map[y as usize][i] == WALL {
                        final_point_x = i as f32 * BOX_SIZE;
                        break;
                    }
                }
            }
        }
        self.animate_shoot(final_point_x, final_point_y);
    }

    fn get_tiles(&self) -> (i32, i32) {
        return (
            round::floor((self.location.x / BOX_SIZE) as f64, 0) as i32,
            round::floor((self.location.y / BOX_SIZE) as f64, 0) as i32,
        );
    }

    pub fn animate_shoot(&self, final_x: f32, final_y: f32) {
        draw_line(
            self.get_center_x(),
            self.get_center_y(),
            final_x,
            final_y,
            5.0,
            VIOLET,
        );
    }
}
