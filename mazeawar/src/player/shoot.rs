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

    pub fn is_target_aligned(&self, target: &Player) -> bool{
        self.location.x == target.location.x || self.location.y == target.location.y
    }

    pub fn is_hit(&self, target: &Player, map: &Map) -> bool{
        let target = (target.location.x / BOX_SIZE, target.location.y / BOX_SIZE); 
        let mut bullet = (self.location.x / BOX_SIZE,  self.location.y / BOX_SIZE);
        let difference = looking_direction_calculation_difference(self.looking_at);
        while bullet != target{
            if map.is_wall(bullet.1, bullet.0){
                return false
            }
            bullet = add_difference(bullet, difference)
        }
        true
    }
}

pub fn looking_direction_calculation_difference(face_dir: Direction) -> (f32,f32){
    match face_dir{
        Direction::UP => {(0.0,-1.0)},
        Direction::DOWN => {(0.0,1.0)},
        Direction::LEFT => {(-1.0,0.0)},
        Direction::RIGHT => {(1.0,0.0)},
    }
}

pub fn add_difference(x:(f32,f32), y:(f32,f32)) -> (f32,f32){
    (x.0+y.0,x.1+y.1)
}
