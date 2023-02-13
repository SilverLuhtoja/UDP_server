#![allow(unused_variables)]
#![allow(unused_assignments)]

use math::round;
use std::f64::consts::PI;

use crate::common::constants::BOX_SIZE;
use crate::map::map::Map;

#[derive(Clone, Debug)]
pub struct Ray {
    pub angle: f32,
    pub distance: f32,
    pub vertical: bool,
}

impl Ray {
    pub fn new(angle: f32, distance: f32, vertical: bool) -> Self {
        Self{angle, distance, vertical}
    }

    /*Get the shortest ray from player position and ray angle*/
    pub fn cast_ray(angle: f32, player_x:f32, player_y:f32, minimap: &Map) -> Ray {
        let vertical_collision_ray:Ray = Ray::get_vertical_collision(angle, &minimap, player_x, player_y);
        let horizontal_collision_ray = Ray::get_horizontal_collision(angle, &minimap, player_x, player_y);
        if horizontal_collision_ray.distance >= vertical_collision_ray.distance {
            return vertical_collision_ray;
        }
        horizontal_collision_ray
    }

    /*get vertical collision ray from given position and angle*/
    pub fn get_vertical_collision(angle:f32, minimap: &Map, player_x:f32, player_y:f32) -> Ray{
        let left: bool = (PI * 3.0) / 2.0 > angle as f64 && angle as f64 > PI / 2.0;
        let right:bool = !left;
    
        let mut first_x: f32 = 0.0;
        let mut horizontal_step:f32 = 0.0;
        if right {
            horizontal_step = BOX_SIZE;
            first_x = round::floor((player_x / BOX_SIZE) as f64, 0) as f32 * BOX_SIZE + BOX_SIZE;
        } else {
            horizontal_step = -BOX_SIZE;
            first_x = round::floor((player_x / BOX_SIZE) as f64, 0) as f32 * BOX_SIZE;
        }
        let first_y:f32 = player_y + (first_x - player_x) * angle.tan();
        let vertical_step:f32 = horizontal_step * angle.tan();
      

        let mut wall:i32 = 0;
        let mut next_x = first_x as f32;
        let mut next_y = first_y as f32;
        while wall == 0 {
            let mut cell_x: f32 = 0.0;
            if right{
                cell_x = round::floor((next_x / BOX_SIZE) as f64, 0) as f32;
            } else {
                cell_x = round::floor((next_x / BOX_SIZE) as f64, 0) as f32 - 1.0;
            }
            let cell_y:f32 = round::floor((next_y / BOX_SIZE) as f64, 0) as f32;
            if minimap.out_of_map_bounce(cell_x, cell_y){
                break;
            }
            wall = minimap.0[cell_y as usize][cell_x as usize];
            if wall == 0 {
                next_x += horizontal_step;
                next_y += vertical_step;
            }
        }
    
        let distance = distance(player_x, player_y, next_x, next_y);
        return Ray::new(angle, distance, true);
    }

    /*get horizontal collision ray from given position and angle*/
    pub fn get_horizontal_collision(angle:f32, minimap: &Map, player_x:f32, player_y:f32) -> Ray{
        let down:bool = angle > 0.0 &&  angle < PI as f32;
        let up: bool = !down;
    
        let mut first_y: f32 = 0.0;
        let mut vertical_step:f32 = 0.0;
        if up {
            vertical_step = -BOX_SIZE;
            first_y = round::floor((player_y / BOX_SIZE - 0.000001) as f64, 0) as f32 * BOX_SIZE;
        } else {
            vertical_step = BOX_SIZE;
            first_y = round::floor((player_y / BOX_SIZE) as f64, 0) as f32 * BOX_SIZE + BOX_SIZE;
        }
        let first_x = player_x + (first_y - player_y) / angle.tan();
        let horizontal_step = vertical_step / angle.tan();

        let mut wall:i32 = 0;
        let mut next_x = first_x as f32;
        let mut next_y = first_y as f32;

        while wall == 0{
            let mut cell_y:f32 = 0.0;
            let cell_x:f32 = round::floor((next_x / BOX_SIZE) as f64, 0) as f32;
            if up {
                cell_y = round::floor((next_y / BOX_SIZE) as f64, 0) as f32 - 1.0;
            } else{
                cell_y = round::floor((next_y / BOX_SIZE) as f64, 0) as f32;
            }
           
            if minimap.out_of_map_bounce(cell_x, cell_y){
                break;
            }
            wall = minimap.0[cell_y as usize][cell_x as usize];
            if wall == 0 {
                next_x += horizontal_step;
                next_y += vertical_step;
            }
        }

        let distance = distance(player_x, player_y, next_x, next_y);
        return Ray::new(angle, distance, false);
    }
}

/*Ray collision distance */
pub fn distance(x1: f32, y1:f32, x2:f32, y2:f32) -> f32 {
    ((x2-x1).powi(2) + (y2-y1).powi(2)).sqrt()
}
