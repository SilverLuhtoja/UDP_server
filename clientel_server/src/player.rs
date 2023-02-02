use macroquad::prelude::*;
use std::f64::consts::PI;

use crate::game_window::GameWindow;
use crate::ray::Ray;

const FOV:f32 = 1.046; //angle of view of rays from player (60 degrees = 30 left + 30 right)-> to_radians(60.0)
// RAD = (deg * PI as f32) / 180.0
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32, //in radians
    pub speed: f32,
    pub radius: f32,
    pub direction_ray_length: f32,
}

impl Player {
    pub fn new(cube_size:f32) -> Player {
        Self {x: 0.0, 
            y:0.0, 
            angle: 0.0, 
            speed:0.0, 
            radius: cube_size/4.0, 
            direction_ray_length: cube_size/2.0, 
        }    
    }
    pub fn set_position(&mut self, game_window: GameWindow) {
        let position = game_window.get_random_empty_space();
        // println!("---------------PLAYER POSITION x, y: {:?}", position);
        self.x = position.0 as f32 * game_window.minimap_cube_size + game_window.minimap_cube_size/2.0;
        self.y = position.1 as f32 * game_window.minimap_cube_size + game_window.minimap_cube_size/2.0;
        // println!("PLAYER POSITION x: {},  y: {}",  self.x,  self.y);
    }

    pub fn draw(&self, game_window: GameWindow) {    
        //draw player on the screen
        draw_circle(self.x, self.y, self.radius, RED);

        //Draw rays from player
        for (i, ray) in self.get_rays(game_window.clone()).iter().enumerate() {
            //on minimap
            draw_line(self.x, self.y, self.x + ray.angle.cos() * ray.distance, self.y + ray.angle.sin() * ray.distance, 1.0, BLACK);

            //visual part:
            let distance:f32 = fix_fish_eye(ray.distance, ray.angle, self.angle);
            let wall_height:f32 = ((game_window.minimap_cube_size * 5.0) / distance) *100.0;
            let mut wall_color:macroquad::color::Color = LIGHTGRAY;
            if ray.vertical {
                wall_color = GRAY;
            }
            //wall
            draw_rectangle(i as f32 + game_window.visual_window_start_x, game_window.visual_window_finish_y/2.0 - wall_height/2.0, 1.0, wall_height, wall_color);
            //floor
            draw_rectangle(i as f32 + game_window.visual_window_start_x, game_window.visual_window_finish_y/2.0 + wall_height/2.0, 1.0, game_window.visual_window_finish_y/2.0 - wall_height/2.0, YELLOW);
            //ceiling
            draw_rectangle(i as f32 + game_window.visual_window_start_x, game_window.visual_window_start_y, 1.0, game_window.visual_window_finish_y/2.0 - wall_height/2.0, WHITE);
        }

        //Draw a line from player to show it`s direction
        draw_line(self.x, self.y, self.x + self.angle.cos() * self.direction_ray_length, self.y + self.angle.sin() * self.direction_ray_length, 1.0, RED);
        
    }
    pub fn step(&mut self){
        self.x += self.angle.cos() * self.speed;
        self.y += self.angle.sin() * self.speed;
    }

    pub fn get_rays(&self, game_window: GameWindow) -> Vec<Ray> {
        // println!("         GET RAYS, player angle {}", self.angle);
        let initial_angle = self.angle - FOV/2.0;
        let number_of_rays:f32 = screen_width() - game_window.visual_window_start_x;
        // let number_of_rays:f32 = 5.0;
        // println!(" number_of_rays {}", number_of_rays);
        let angle_step:f32 = FOV / number_of_rays;
        // println!(" angle_step {}", angle_step);
        let mut result: Vec<Ray> = Vec::new();
        let mut i = 0;
        while i < number_of_rays as i32 {
            let angle:f32 = initial_angle + i as f32 * angle_step;
            let one_ray:Ray = Ray::cast_ray(angle, game_window.clone(), self.x, self.y);
            result.push(one_ray);
            i += 1;
        }
        // println!(" result {:?}", result[0].distance);
        return result;
    }
}

pub fn fix_fish_eye(distance: f32, angle: f32, player_angle: f32) -> f32 {
    let diff = angle - player_angle;
    return distance * diff.cos();
}