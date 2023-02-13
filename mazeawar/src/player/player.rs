use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

use crate::common::constants::BOX_SIZE;
use crate::map::game_window::GameWindow;
use crate::map::map::Map;
use crate::utils::point::Point;
use crate::utils::ray::Ray;

const FOV: f32 = 1.046; //angle of view of rays from player (60 degrees = 30 left + 30 right)-> to_radians(60.0)

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Player {
    pub location: Point,
    pub looking_at: Direction,
    pub username: String,
    pub score: i32,
}


impl Player {
    pub fn new(location: Point) -> Self {
        Self {
            location,
            looking_at: Direction::UP,
            username: "".to_string(),
            score: 0,
        }        
    }

    pub fn set_position(&mut self, point: Point){
        self.location = point;
    }

    pub fn get_center_x(&self) -> f32 {
        self.location.x + BOX_SIZE / 2.0
    }

    pub fn get_center_y(&self) -> f32 {
        self.location.y + BOX_SIZE / 2.0
    }

    pub fn draw(&self, game_window: &GameWindow, map: &Map, is_shot: bool){
        // Draw rays from player on minimap and visual part
        for (i, ray) in self.get_rays(game_window.visual_window_start_x, map).iter().enumerate() {
            //on minimap
            let start_x: f32 = self.location.x + BOX_SIZE / 2.0;
            let start_y: f32 = self.location.y + BOX_SIZE / 2.0;
            let player_angle: f32 = looking_direction_to_radians(self.looking_at);
            draw_line(start_x, start_y, start_x + ray.angle.cos() * ray.distance, start_y + ray.angle.sin() * ray.distance, 1.0, BEIGE);

            //visual part:
            let distance = fix_fish_eye(ray.distance, ray.angle, player_angle);
            let wall_height = ((BOX_SIZE * 5.0) / distance) * 150.0;

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
            let x = (game_window.visual_window_start_x + game_window.visual_window_finish_x) / 2.0;
            draw_line(x, game_window.visual_window_finish_y, x, game_window.visual_window_finish_y / 2.0, 5.0, GREEN);
        }
        //line to separate map and visual
        draw_line(game_window.visual_window_start_x, 0.0, game_window.visual_window_start_x, screen_height(), 1.0, BLACK);
        //draw player on the minimap
        draw_circle(self.location.x + BOX_SIZE /2.0, self.location.y + BOX_SIZE/2.0, BOX_SIZE/4.0, GREEN);
        self.draw_facing_indicator();
    }

    pub fn draw_enemy(&self, enemy: Player,screen: &GameWindow, visible: bool ){
        //minimap
        draw_circle(enemy.location.x + BOX_SIZE /2.0, enemy.location.y + BOX_SIZE/2.0, BOX_SIZE/4.0, RED);
        enemy.draw_facing_indicator();

        //visual part    
        if visible {
            let sx = enemy.location.x - self.location.x;
            let sy = enemy.location.y - self.location.y;
            let cs = looking_direction_to_radians(self.looking_at).cos();
            let sn = looking_direction_to_radians(self.looking_at).sin();
            let a = sy*cs+sx*sn;
            let b = sx*cs-sy*sn;
            let distance = (sx.powi(2) + sy.powi(2)).sqrt();
    
            let (width_center,height_center) = screen.get_visual_screen_center_point();
    
            let screen_sx = (a*108.0 * 4.0/b) + width_center;
            let screen_sy = (1.0/b) +  height_center;
            draw_circle(screen_sx, screen_sy, (BOX_SIZE/distance) * 100.0, RED);
        }
    }

    pub fn draw_facing_indicator(&self) {
        let indicator_size: f32 = 5.0;
        match self.looking_at {
            Direction::UP => draw_rectangle(self.get_center_x() - indicator_size / 2.0 , self.get_center_y() - BOX_SIZE / 2.0, indicator_size, indicator_size, RED),
            Direction::DOWN => draw_rectangle(self.get_center_x() - indicator_size / 2.0, self.get_center_y() + BOX_SIZE / 4.0, indicator_size, indicator_size, RED),
            Direction::LEFT => draw_rectangle(self.get_center_x() - BOX_SIZE / 2.0, self.get_center_y() - indicator_size / 2.0, indicator_size, indicator_size, RED),
            Direction::RIGHT => draw_rectangle(self.get_center_x() + BOX_SIZE / 4.0, self.get_center_y() - indicator_size / 2.0, indicator_size, indicator_size, RED)
        }
    }

    /* Get 60degree FOV (field of view) rays from player position */
    pub fn get_rays(&self, visual_window_start_x: f32, map: &Map) -> Vec<Ray> {
        let player_angle: f32 = looking_direction_to_radians(self.looking_at);
        let initial_angle = player_angle - FOV / 2.0;
        let number_of_rays: f32 = screen_width() - visual_window_start_x;
        let angle_step: f32 = FOV / number_of_rays;
        let mut result: Vec<Ray> = Vec::new();
        let mut i = 0;
        while i < number_of_rays as i32 {
            let angle: f32 = initial_angle + i as f32 * angle_step;
            let one_ray: Ray = Ray::cast_ray(angle, self.location.x + BOX_SIZE / 2.0, self.location.y + BOX_SIZE / 2.0, &map);
            result.push(one_ray);
            i += 1;
        }
        return result;
    }
}

pub fn looking_direction_to_radians(direction: Direction) -> f32 {
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
