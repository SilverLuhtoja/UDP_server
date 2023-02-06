use macroquad::prelude::*;
use math::round;
use serde::{Deserialize, Serialize};

use crate::ray::*;
use crate::map::*;
use crate::miniquad::gl::user_addr_t;

const FOV: f32 = 1.046; //angle of view of rays from player (60 degrees = 30 left + 30 right)-> to_radians(60.0)

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Player {
    pub location: Point,
    pub looking_at: Direction,
    pub username: String,
    pub score: i32,
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

pub const BOX_SIZE: f32 = 20.0;

impl Player {
    pub fn new(location: Point) -> Self {
        Self {
            location,
            looking_at: Direction::UP,
            username: "default name".to_string(),
            score: 15,
        }
    }

    pub fn draw(&self, me: bool, game_window: GameWindow, map: Map, is_shot: bool) {
        let mut player_color: macroquad::color::Color = RED;
        if me {
            let mut rays: Vec<Ray> = Vec::new();
            //player color on the minimap
            player_color = GREEN;

            //line to separate map and visual
            draw_line(game_window.visual_window_start_x, 0.0, game_window.visual_window_start_x, screen_height(), 1.0, BLACK);

            // Draw rays from player on minimap and visual part
            for (i, ray) in self.get_rays(game_window.visual_window_start_x, map).iter().enumerate() {
                if ray.angle == get_angle(self.looking_at) {
                    rays.push(ray.clone());
                }
                //on minimap
                let start_x: f32 = self.location.x + BOX_SIZE / 2.0;
                let start_y: f32 = self.location.y + BOX_SIZE / 2.0;
                let player_angle: f32 = get_angle(self.looking_at);
                draw_line(start_x, start_y, start_x + ray.angle.cos() * ray.distance, start_y + ray.angle.sin() * ray.distance, 1.0, BEIGE);

                //visual part:
                let distance: f32 = fix_fish_eye(ray.distance, ray.angle, player_angle);
                let wall_height: f32 = ((BOX_SIZE * 5.0) / distance) * 70.0;
                let mut wall_color: macroquad::color::Color = LIGHTGRAY;
                if ray.vertical {
                    wall_color = GRAY;
                }
                //wall
                draw_rectangle(i as f32 + game_window.visual_window_start_x, game_window.visual_window_finish_y / 2.0 - wall_height / 2.0, 1.0, wall_height, wall_color);
                //floor
                draw_rectangle(i as f32 + game_window.visual_window_start_x, game_window.visual_window_finish_y / 2.0 + wall_height / 2.0, 1.0, game_window.visual_window_finish_y / 2.0 - wall_height / 2.0, BEIGE);
                //ceiling
                draw_rectangle(i as f32 + game_window.visual_window_start_x, game_window.visual_window_start_y, 1.0, game_window.visual_window_finish_y / 2.0 - wall_height / 2.0, WHITE);
            }
            if is_shot {
                // draw_line();
                draw_rectangle(game_window.visual_window_start_x, game_window.visual_window_start_y, screen_width(), screen_height(), ORANGE);
            }
        }
        let x = game_window.visual_window_start_x + game_window.visual_window_finish_x / 2.0;
        draw_line(x, game_window.visual_window_finish_y, x+rays[0].distance, game_window.visual_window_start_y, 50.0, RED);
        //draw player on the minimap
        draw_circle(self.location.x + BOX_SIZE / 2.0, self.location.y + BOX_SIZE / 2.0, BOX_SIZE / 4.0, player_color);
        self.draw_facing_indicator();
    }

    pub fn draw_facing_indicator(&self) {
        let middle_offset: f32 = 7.5;
        match self.looking_at {
            Direction::UP => draw_rectangle(self.location.x + middle_offset, self.get_center_y() - BOX_SIZE / 2.0, 5.0, 5.0, RED),
            Direction::DOWN => draw_rectangle(self.location.x + middle_offset, self.get_center_y() + BOX_SIZE / 2.0, 5.0, 5.0, RED),
            Direction::LEFT => draw_rectangle(self.get_center_x() - BOX_SIZE / 2.0, self.location.y + middle_offset, 5.0, 5.0, RED),
            Direction::RIGHT => draw_rectangle(self.get_center_x() + BOX_SIZE / 2.0, self.location.y + middle_offset, 5.0, 5.0, RED)
        }
    }

    pub fn set_postion(&mut self, point: Point) {
        self.location = point;
    }
    pub fn get_center_x(&self) -> f32 {
        self.location.x + BOX_SIZE / 2.0
    }
    pub fn get_center_y(&self) -> f32 {
        self.location.y + BOX_SIZE / 2.0
    }

    pub fn turn_left(&mut self) {
        match self.looking_at {
            Direction::UP => self.looking_at = Direction::LEFT,
            Direction::DOWN => self.looking_at = Direction::RIGHT,
            Direction::LEFT => self.looking_at = Direction::DOWN,
            Direction::RIGHT => self.looking_at = Direction::UP,
        }
    }
    pub fn turn_right(&mut self) {
        match self.looking_at {
            Direction::UP => self.looking_at = Direction::RIGHT,
            Direction::DOWN => self.looking_at = Direction::LEFT,
            Direction::LEFT => self.looking_at = Direction::UP,
            Direction::RIGHT => self.looking_at = Direction::DOWN,
        }
    }
    pub fn step(&mut self, step: f32, map: Vec<Vec<i32>>) {
        let mut new_point: Point = self.location.clone();
        match self.looking_at {
            Direction::LEFT => {
                new_point.x -= step;
            }
            Direction::RIGHT => {
                new_point.x += step;
            }
            Direction::UP => {
                new_point.y -= step;
            }
            Direction::DOWN => {
                new_point.y += step;
            }
        }
        if can_step(new_point, map) {
            self.location = new_point;
        }
    }

    /* Get 60degree FOV (field of view) rays from player position */
    pub fn get_rays(&self, visual_window_start_x: f32, map: Map) -> Vec<Ray> {
        let player_angle: f32 = get_angle(self.looking_at);
        let initial_angle = player_angle - FOV / 2.0;
        let number_of_rays: f32 = screen_width() - visual_window_start_x;
        let angle_step: f32 = FOV / number_of_rays;
        let mut result: Vec<Ray> = Vec::new();
        let mut i = 0;
        while i < number_of_rays as i32 {
            let angle: f32 = initial_angle + i as f32 * angle_step;
            let one_ray: Ray = Ray::cast_ray(angle, self.location.x + BOX_SIZE / 2.0, self.location.y + BOX_SIZE / 2.0, map.clone());
            result.push(one_ray);
            i += 1;
        }
        return result;
    }
//shooting
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
        return (round::floor((self.location.x / BOX_SIZE) as f64, 0) as i32,
                round::floor((self.location.y / BOX_SIZE) as f64, 0) as i32);
    }
    pub fn animate_shoot(&self, final_x: f32, final_y: f32) {
        draw_line(self.get_center_x(), self.get_center_y(), final_x, final_y, 5.0, VIOLET);
    }
}
/*get radians from direction*/
pub fn get_angle(direction: Direction) -> f32 {
    match direction {
        Direction::LEFT => 3.14,
        Direction::RIGHT => 0.0,
        Direction::UP => 4.71,
        Direction::DOWN => 1.57,
    }
}

pub fn fix_fish_eye(distance: f32, angle: f32, player_angle: f32) -> f32 {
    let diff = angle - player_angle;
    return distance * diff.cos();
}

/*Check collision with walls*/
pub fn can_step(new_location: Point, map: Vec<Vec<i32>>) -> bool {
    let x = new_location.x / BOX_SIZE;
    let y = new_location.y / BOX_SIZE;
    return map[y as usize][x as usize] == 0;
}