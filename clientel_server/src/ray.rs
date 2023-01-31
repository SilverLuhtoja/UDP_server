use crate::game_window::GameWindow;
use math::round;
use std::f64::consts::PI;

pub struct Ray {
    pub angle: f32,
    pub distance: f32,
    pub vertical: bool,
}

impl Ray {
    pub fn new(angle: f32, distance: f32, vertical: bool) -> Self {
        Self{angle, distance, vertical}
    }

    pub fn cast_ray(angle: f32, game_window: GameWindow, player_x:f32, player_y:f32) -> Ray {
        let vertical_collision_ray:Ray = Ray::get_vertical_collision(angle, game_window.clone(), player_x, player_y);
        let horizontal_collision_ray = Ray::get_horizontal_collision(angle, game_window.clone(), player_x, player_y);
        if horizontal_collision_ray.distance >= vertical_collision_ray.distance {
            return vertical_collision_ray;
        }
        horizontal_collision_ray
    }
    
    pub fn get_vertical_collision(angle:f32, game_window: GameWindow, player_x:f32, player_y:f32) -> Ray{
        let left: bool = (PI * 3.0) / 2.0 > angle as f64 && angle as f64 > PI / 2.0;
        let right:bool = !left;
        // println!("FACING right? {}, angle {}", right, angle);
    
        let mut first_x: f32 = 0.0;
        let mut horizontal_step:f32 = 0.0;
        if right {
            horizontal_step = game_window.minimap_cube_size;
            first_x = round::floor((player_x / game_window.minimap_cube_size) as f64, 0) as f32 * game_window.minimap_cube_size + game_window.minimap_cube_size;
        } else {
            horizontal_step = -game_window.minimap_cube_size;
            first_x = round::floor((player_x / game_window.minimap_cube_size) as f64, 0) as f32 * game_window.minimap_cube_size;
        }

        let first_y:f32 = player_y + (first_x - player_x) * angle.tan();
        let vertical_step:f32 = horizontal_step * angle.tan();
      

        let mut wall:i32 = 0;
        let mut next_x = first_x as f32;
        let mut next_y = first_y as f32;
        // println!( "horizontal_step {}, vertical_step {}", horizontal_step, vertical_step);
        // println!( "playerX {}, playerY {},  next_x {}, next_y: {}", player_x, player_y, next_x, next_y);
        while wall == 0 {
            let mut cell_x: f32 = 0.0;
            if right{
                cell_x = round::floor((next_x / game_window.minimap_cube_size) as f64, 0) as f32;
            } else {
                cell_x = round::floor((next_x / game_window.minimap_cube_size) as f64, 0) as f32 - 1.0;
            }
            let cell_y:f32 = round::floor((next_y / game_window.minimap_cube_size) as f64, 0) as f32;
            // println!("---------------PLAYER POSITION x: {}, y: {}", player_x , player_y);
            // println!( "cell POSITION: {}, {}", cell_x, cell_y);
            if game_window.out_of_map_bounce(cell_x, cell_y){
                break;
            }
            wall = game_window.minimap[cell_y as usize][cell_x as usize];
            // println!( "wall: {} {:?}", wall, game_window.minimap[cell_y as usize]);
            if wall == 0 {
                next_x += horizontal_step;
                next_y += vertical_step;
                // println!( "GOING NEXT ROUND: next_x {} next_y {}", next_x, next_y);
            }
        }
      
        let distance = distance(player_x, player_y, next_x, next_y);
        // println!( "GOING OUT with distance {}", distance);
        return Ray::new(angle, distance, true);
    }

    pub fn get_horizontal_collision(angle:f32, game_window: GameWindow, player_x:f32, player_y:f32) -> Ray{
        let down:bool = angle > 0.0 &&  angle < PI as f32;
        let up: bool = !down;
        // println!("FACING ? {}, angle {}", up, angle);
    
        let mut first_y: f32 = 0.0;
        let mut vertical_step:f32 = 0.0;
        if up {
            vertical_step = -game_window.minimap_cube_size;
            first_y = round::floor((player_y / game_window.minimap_cube_size - 0.000001) as f64, 0) as f32 * game_window.minimap_cube_size;
        } else {
            vertical_step = game_window.minimap_cube_size;
            first_y = round::floor((player_y / game_window.minimap_cube_size) as f64, 0) as f32 * game_window.minimap_cube_size + game_window.minimap_cube_size;
        }

        let first_x = player_x + (first_y - player_y) / angle.tan();
        let horizontal_step = vertical_step / angle.tan();


        let mut wall:i32 = 0;
        let mut next_x = first_x as f32;
        let mut next_y = first_y as f32;
        // println!( "vertical_step {}, horizontal_step {}", vertical_step, horizontal_step);
        // println!( "playerX {}, playerY {},  next_x {}, next_y: {}", player_x, player_y, next_x, next_y);
        while wall == 0{
            let mut cell_y:f32 = 0.0;
            let mut cell_x:f32 = round::floor((next_x / game_window.minimap_cube_size) as f64, 0) as f32;

            if up {
                cell_y = round::floor((next_y / game_window.minimap_cube_size) as f64, 0) as f32 - 1.0;
            } else{
                cell_y = round::floor((next_y / game_window.minimap_cube_size) as f64, 0) as f32;
            }
            // println!("---------------PLAYER POSITION x: {}, y: {}", (player_x - game_window.minimap_cube_size/2.0 - game_window.minimap_start_x) / game_window.minimap_cube_size , (player_y - game_window.minimap_cube_size/2.0 - game_window.minimap_start_y) / game_window.minimap_cube_size);
            // println!( "cell POSITION: {}, {}", cell_x, cell_y);
            if game_window.out_of_map_bounce(cell_x, cell_y){
                break;
            }
            wall = game_window.minimap[cell_y as usize][cell_x as usize];
            // println!( "wall: {} {:?}", wall, game_window.minimap[cell_y as usize]);
            if wall == 0 {
                next_x += horizontal_step;
                next_y += vertical_step;
                // println!( "GOING NEXT ROUND: next_x {} next_y {}", next_x, next_y);
            }
        }
        let distance = distance(player_x, player_y, next_x, next_y);
        // println!( "GOING OUT with distance {}", distance);
        return Ray::new(angle, distance, false);
    }
}

pub fn distance(x1: f32, y1:f32, x2:f32, y2:f32) -> f32 {
    // let delta_x = x2 - x1;
	// let delta_y = y2 - y1;
    ((x2-x1).powi(2) + (y2-y1).powi(2)).sqrt()
    // (delta_x * delta_x + delta_y * delta_y).sqrt()
}