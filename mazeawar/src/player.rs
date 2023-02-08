use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

use crate::ray::*;
use crate::map::*;

const FOV:f32 = 1.046; //angle of view of rays from player (60 degrees = 30 left + 30 right)-> to_radians(60.0)

#[derive(Clone,Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum  Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(Clone,Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Point{
    pub x: f32,
    pub y: f32
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Player{
    pub location: Point,
    pub looking_at:  Direction,
    pub username: String,
    pub score: i32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero()->Self{
        Self { x: 0.0, y: 0.0 }
    }

    pub fn is_moved(&self)->bool{
        !(self.x == 0.0 && self.y == 0.0)
    }
}

pub const BOX_SIZE:f32 = 20.0;

impl Player{
    pub fn new(location: Point) -> Self {
        Self{
            location,
            looking_at: Direction::UP,
            username: "default name".to_string(),
            score: 15,
        }        
    }
    pub fn set_loc(&mut self, loc: Point){
        self.location = loc;
    }

    pub fn draw(&self, game_window: GameWindow, map: Map){
            let player_color:macroquad::color::Color = GREEN;

            //line to separate map and visual
            draw_line(game_window.visual_window_start_x, 0.0, game_window.visual_window_start_x, screen_height(), 1.0, BLACK);
            
            // Draw rays from player on minimap and visual part
            for (i, ray) in self.get_rays(game_window.visual_window_start_x, map.clone()).iter().enumerate() {
                //on minimap
                let start_x:f32 = self.location.x+ BOX_SIZE /2.0;
                let start_y:f32 = self.location.y+ BOX_SIZE /2.0;
                let player_angle:f32 = get_angle(self.looking_at);

                //  This is generating raycone, so i could theoretically check if enemy player location is inside this ????
                draw_line(start_x, start_y, start_x + ray.angle.cos() * ray.distance, start_y + ray.angle.sin() * ray.distance, 1.0, BEIGE);

                let ray_x = start_x + ray.angle.cos() * ray.distance;
                let ray_y = start_y + ray.angle.sin() * ray.distance;

                
                //visual part:
                let distance:f32 = fix_fish_eye(ray.distance, ray.angle, player_angle);
                let wall_height:f32 = ((BOX_SIZE * 5.0) / distance) *70.0;
                let mut wall_color:macroquad::color::Color = LIGHTGRAY;
                if ray.vertical {
                    wall_color = GRAY;
                }
            // if self.casting_on_what(ray_x, ray_y, &map) == 2 {
            //     println!("CASTING ON PLAYER");
            //     draw_rectangle(i as f32 + game_window.visual_window_start_x, game_window.visual_window_finish_y/2.0 - wall_height/2.0, 1.0, wall_height, RED);
            // }else{

                //wall
                draw_rectangle(i as f32 + game_window.visual_window_start_x, game_window.visual_window_finish_y/2.0 - wall_height/2.0, 1.0, wall_height, wall_color);
                //floor
                draw_rectangle(i as f32 + game_window.visual_window_start_x, game_window.visual_window_finish_y/2.0 + wall_height/2.0, 1.0, game_window.visual_window_finish_y/2.0 - wall_height/2.0, BEIGE);
                //ceiling
                draw_rectangle(i as f32 + game_window.visual_window_start_x, game_window.visual_window_start_y, 1.0, game_window.visual_window_finish_y/2.0 - wall_height/2.0, WHITE);
            // }
        }
        //draw player on the minimap
        draw_circle(self.location.x + BOX_SIZE /2.0, self.location.y + BOX_SIZE/2.0, BOX_SIZE/4.0, player_color);
        self.draw_facing_indicator();
        
    }

    pub fn casting_on_what(&self, ray_x:f32,ray_y:f32,map: &Map) -> i32 {
        map.0[(ray_x/20.0) as usize][(ray_y/20.0) as usize] 
    }

    pub fn draw_enemy(&self, enemy: &Player,screen: &GameWindow){
        draw_circle(enemy.location.x + BOX_SIZE /2.0, enemy.location.y + BOX_SIZE/2.0, BOX_SIZE/4.0, RED);

        let sx = enemy.location.x - self.location.x;
        let sy = enemy.location.y - self.location.y;
        let cs = get_angle(self.looking_at).cos();
        let sn = get_angle(self.looking_at).sin();
        let a = sy*cs+sx*sn;
        let b = sx*cs-sy*sn;
        let distance = (sx.powi(2) + sy.powi(2)).sqrt();

        let (width_center,height_center) = screen.get_visual_screen_center_point();
        
        let screen_sx = (a*108.0/b) + width_center;
        let screen_sy = (1.0/b) +  height_center;
        draw_circle(screen_sx, screen_sy, (BOX_SIZE/distance) * 100.0, RED);
        // let distance = distance(enemy.location.x, self.location.x, enemy.location.y, self.location.y);
        // println!("DIstance from enemy{}", distance);
        // println!("DIstance from enemy {} {}", a, b);
    }
   
    pub fn draw_facing_indicator(&self){
        let middle_offset:f32 = 7.5;
        match self.looking_at {
            Direction::UP => draw_rectangle(self.location.x + middle_offset ,self.get_center_y() - BOX_SIZE /2.0, 5.0,  5.0,  RED),
            Direction::DOWN => draw_rectangle(self.location.x + middle_offset , self.get_center_y() + BOX_SIZE/2.0, 5.0,  5.0,  RED),
            Direction::LEFT => draw_rectangle(self.get_center_x() - BOX_SIZE/2.0 , self.location.y + middle_offset, 5.0,  5.0,  RED),
            Direction::RIGHT => draw_rectangle(self.get_center_x() + BOX_SIZE/2.0 , self.location.y + middle_offset, 5.0,  5.0,  RED)
        }
    }

    pub fn set_postion(&mut self, point: Point){
        self.location = point;
    }
    pub fn get_center_x(&self) -> f32{
        self.location.x + BOX_SIZE/2.0
    }
    pub fn get_center_y(&self) -> f32{
        self.location.y + BOX_SIZE/2.0
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
    pub fn step(&mut self, step: f32, map: Vec<Vec<i32>>){
        let mut new_point: Point = self.location.clone();
        match self.looking_at {
            Direction::LEFT => {
                new_point.x -= step;
            },
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
        if can_step(new_point, map){
            self.location = new_point;
        }
    }

    /* Get 60degree FOV (field of view) rays from player position */
    pub fn get_rays(&self, visual_window_start_x:f32, map: Map) -> Vec<Ray> {
        let player_angle:f32 = get_angle(self.looking_at);
        let initial_angle = player_angle - FOV/2.0;
        let number_of_rays:f32 = screen_width() - visual_window_start_x;
        let angle_step:f32 = FOV / number_of_rays;
        let mut result: Vec<Ray> = Vec::new();
        let mut i = 0;
        while i < number_of_rays as i32 {
            let angle:f32 = initial_angle + i as f32 * angle_step;
            let one_ray:Ray = Ray::cast_ray(angle, self.location.x+ BOX_SIZE /2.0, self.location.y+ BOX_SIZE /2.0, map.clone());
            result.push(one_ray);
            i += 1;
        }
        return result;
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
pub fn can_step(new_location: Point, map: Vec<Vec<i32>>)-> bool {
    let x = new_location.x / BOX_SIZE;
    let y = new_location.y / BOX_SIZE;
    return map[y as usize][x as usize] == 0;
}