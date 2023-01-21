use macroquad::prelude::*;
use clientel_server::*;
use math::round;
use std::f64::consts::PI;

const PLAYER_SIZE:f32 = 10.0;
const PLAYER_RAY_LENGTH:f32 = PLAYER_SIZE * 2.0;
const FOV:f32 = 1.046; //angle of view of rays from player (60 degrees = 30 left + 30 right)-> to_radians(60.0)


pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub speed: f32,
    pub radius: f32,
}

impl Player {
    pub fn new(x:f32, y:f32, angle:f32, speed:f32) -> Player {
        Self {x, y, angle, speed, radius: PLAYER_SIZE/2.0}    
    }
    pub fn draw(&self, game_board: GameBoard) {    
        //draw player on the screen
        draw_circle(self.x, self.y, self.radius, RED);

        //Draw rays from player
        for (i, ray) in self.get_rays(game_board.clone()).iter().enumerate() {
            //minimap
            draw_line(self.x, self.y, self.x + ray.angle.cos() * ray.distance, (self.y + ray.angle.sin() * ray.distance) + self.radius, 1.0, BLACK);

            //visual part
            let distance:f32 = ray.distance;
            let wall_height:f32 = ((game_board.setup.cube_size * 5.0) / distance) *277.0;
            let mut wall_color:macroquad::color::Color = LIGHTGRAY;
            if ray.vertical {
                wall_color = GRAY;
            }
            //wall
            draw_rectangle(i as f32 + 20.0, game_board.setup.finish_y/2.0 - wall_height/2.0, 1.0, wall_height, wall_color);
            //floor
            draw_rectangle(i as f32 + 20.0, game_board.setup.finish_y/2.0 + wall_height/2.0, 1.0, screen_height()/2.0 - wall_height/2.0, YELLOW);
            //ceiling
            draw_rectangle(i as f32 + 20.0, 20.0, 1.0, game_board.setup.finish_y/2.0 - wall_height/2.0, WHITE);

        }
        
        //Draw a line from player to show it`s direction
        draw_line(self.x, self.y, self.x + self.angle.cos() * PLAYER_RAY_LENGTH, self.y + self.angle.sin() * PLAYER_RAY_LENGTH, 1.0, RED);
        

    }
    pub fn move_player(&mut self, game_board: GameBoard){
        println!("Move player {}, {}", self.x, self.y);
        if self.can_move(game_board.clone()){
            self.x += self.angle.cos() * self.speed;
            self.y += self.angle.sin() * self.speed;
        }
    }
    pub fn can_move(&self, game_board: GameBoard) -> bool {
        // let next_x = self.x + self.angle.cos() * self.speed;
        // let next_y = self.y + self.angle.sin() * self.speed;
        // println!("NEXT STEP {}, {}", next_x, next_y);
        // let mut results:Vec<bool> = Vec::new();

        // // println!("CHECKING ALL WALLS {:?}", game_board.wall_coordinates);
        // for wall in game_board.wall_coordinates {
            
        //     //TODO NOT WORKING!!!
        //     //find only nearest walls
        //     // if next_x - wall.0 + game_board.setup.cube_size <= self.radius && 
        //     // next_y - wall.1 + game_board.setup.cube_size <= self.radius || wall.1 - next_y <= self.radius
        //     // || wall.0 - next_x <= self.radius && next_y - wall.1 + game_board.setup.cube_size <= self.radius || wall.1 - next_y <= self.radius {

        //     //     println!("NEAREST WALL: {:?}", wall);
        //     //     let circle_distance_x = (next_x - wall.0).abs();
        //     //     let circle_distance_y = (next_y - wall.1).abs();
            
        //     //     if circle_distance_x > (game_board.setup.cube_size/2.0 + self.radius) { 
        //     //         results.push(false); 
        //     //     }
        //     //     if circle_distance_y > (game_board.setup.cube_size/2.0 + self.radius) { 
        //     //         results.push(false); 
        //     //     }
        //     //     if circle_distance_x <= (game_board.setup.cube_size/2.0) { 
        //     //         results.push(true); } 
        //     //     if circle_distance_y <= (game_board.setup.cube_size/2.0) { 
        //     //         results.push(true); }
            
        //     //     let corner_distance_sq = (circle_distance_x - game_board.setup.cube_size/2.0).powi(2) +
        //     //                             (circle_distance_y - game_board.setup.cube_size/2.0).powi(2);
            
        //     //     results.push(corner_distance_sq <= ((self.radius).powi(2)));
        //     // }
        // }
        // for result in results {
        //     if result == false{
        //         return false;
        //     }
        // }
        return true;
    }

    pub fn get_rays(&self, game_board: GameBoard) -> Vec<Ray> {
        // println!("         GET RAYS, player angle {}", self.angle);
        let initial_angle = self.angle - FOV/2.0;
        let number_of_rays:f32 = 2500.0; //TODO CHANGE!!!
        // println!(" number_of_rays {}", number_of_rays);
        let angle_step:f32 = FOV / number_of_rays;
        // println!(" angle_step {}", angle_step);
        let mut result: Vec<Ray> = Vec::new();
        let mut i = 0;
        while i < number_of_rays as i32 {
            let angle:f32 = initial_angle + i as f32 * angle_step;
            let one_ray:Ray = Ray::cast_ray(angle, game_board.clone(), self.x, self.y);
            result.push(one_ray);
            i += 1;
        }
        // println!(" result {:?}", result[0].distance);
        return result;
    }


}

pub struct Ray {
    pub angle: f32,
    pub distance: f32,
    pub vertical: bool,
}

impl Ray {
    pub fn new(angle: f32, distance: f32, vertical: bool) -> Self {
        Self{angle, distance, vertical}
    }

    pub fn cast_ray(angle: f32, game_board: GameBoard, player_x:f32, player_y:f32) -> Ray {
        let vertical_collision_ray:Ray = Ray::get_vertical_collision(angle, game_board.clone(), player_x, player_y);
        let horizontal_collision_ray = Ray::get_horizontal_collision(angle, game_board.clone(), player_x, player_y);
        if horizontal_collision_ray.distance >= vertical_collision_ray.distance {
            return vertical_collision_ray
        }
        horizontal_collision_ray
    }
    
    pub fn get_vertical_collision(angle:f32, game_board: GameBoard, player_x:f32, player_y:f32) -> Ray{
        let right:bool = (round::floor((angle as f64 - PI/2.0) / PI , 2) % 2.0 ).abs() > 1.0;
        // println!("FACING RIGHT? {}, angle {}", right, (round::floor((angle as f64 - PI/2.0) / PI , 2) % 2.0 ).abs());
    
        let mut first_x: f32 = 0.0;
        let mut horizontal_step:f32 = 0.0;
        if right {
            horizontal_step = game_board.setup.cube_size;
            first_x = round::floor((player_x / game_board.setup.cube_size) as f64, 2) as f32 * game_board.setup.cube_size + game_board.setup.cube_size;
        } else {
            horizontal_step = -game_board.setup.cube_size;
            first_x = round::floor((player_x / game_board.setup.cube_size) as f64, 2) as f32 * game_board.setup.cube_size;
        }

        let first_y:f32 = round::floor((player_y + (first_x - player_x) * angle.tan()) as f64, 0) as f32;
        let vertical_step:f32 = round::floor((horizontal_step * angle.tan()) as f64, 0) as f32;
      

        let mut wall:i32 = 0;
        let mut next_x = first_x as f32;
        let mut next_y = first_y as f32;
        // println!( "horizontal_step {}, vertical_step {}", horizontal_step, vertical_step);
        // println!( "playerX {}, playerY {},  next_x {}, next_y: {}", player_x, player_y, next_x, next_y);
        while wall == 0 {
            let mut cell_x: f32 = 0.0;
            if right{
                cell_x = round::floor((next_x / game_board.setup.cube_size) as f64, 0) as f32;
            } else {
                cell_x = round::floor((next_x / game_board.setup.cube_size) as f64, 0) as f32 - 1.0;
            }
            let cell_y:f32 = round::floor(((next_y - game_board.setup.start_y) / game_board.setup.cube_size) as f64, 0) as f32;

            // println!( "cell POSITION: {}, {}", cell_x, cell_y);
            if game_board.out_of_map_bounce(cell_x, cell_y){
                break;
            }
            wall = game_board.board[cell_y as usize][cell_x as usize];
            // println!( "wall: {} {:?}", wall, game_board.board[cell_y as usize]);
            if wall == 0 {
                
                next_x += round::floor(vertical_step as f64, 0) as f32;
                next_y += round::floor(horizontal_step as f64, 0) as f32;
                // println!( "GOING NEXT ROUND: next_x {} next_y {}", next_x, next_y);
            }
        }
      
        let distance = distance(player_x, player_y, next_x, next_y);
        // println!( "GOING OUT with distance {}", distance);
        return Ray::new(angle, distance, true);
    }

    pub fn get_horizontal_collision(angle:f32, game_board: GameBoard, player_x:f32, player_y:f32) -> Ray{
        let up:bool = (round::floor(angle as f64 / PI , 2) % 2.0 ).abs() >= 1.0;
        // println!("FACING up? {}, angle {}", up, (round::floor(angle as f64 / PI , 2) % 2.0 ).abs());
    
        let mut first_y: f32 = 0.0;
        let mut vertical_step:f32 = 0.0;
        if up {
            vertical_step = -game_board.setup.cube_size;
            first_y = round::floor((player_y / game_board.setup.cube_size) as f64, 2) as f32 * game_board.setup.cube_size;
        } else {
            vertical_step = game_board.setup.cube_size;
            first_y = round::floor((player_y / game_board.setup.cube_size) as f64, 2) as f32 * game_board.setup.cube_size + game_board.setup.cube_size;
        }

        let first_x = round::floor((player_x + (first_y - player_y) / angle.tan()) as f64, 0) as f32;
        let horizontal_step = round::floor((vertical_step / angle.tan())as f64, 0) as f32;


        let mut wall:i32 = 0;
        let mut next_x = first_x as f32;
        let mut next_y = first_y as f32;

        // println!( "vertical_step {}, horizontal_step {}", vertical_step, horizontal_step);
        // println!( "playerX {}, playerY {},  next_x {}, next_y: {}", player_x, player_y, next_x, next_y);
        while wall == 0{
            let mut cell_y:f32 = 0.0;
            let mut cell_x:f32 = round::floor((next_x/ game_board.setup.cube_size) as f64, 0) as f32;
            if up {
                cell_y = round::floor(((next_y - game_board.setup.start_y)/ game_board.setup.cube_size) as f64, 0) as f32 - 1.0;
            } else{
                cell_y = round::floor(((next_y - game_board.setup.start_y)/ game_board.setup.cube_size) as f64, 0) as f32;
            }
            //  println!( "cell POSITION: {}, {}", cell_x, cell_y);
            if game_board.out_of_map_bounce(cell_x, cell_y){
                break;
            }
            wall = game_board.board[cell_y as usize][cell_x as usize];
            // println!( "wall: {} {:?}", wall, game_board.board[cell_y as usize]);
            if wall == 0 {
                    
                next_x += round::floor(vertical_step as f64, 0) as f32;
                next_y += round::floor(horizontal_step as f64, 0) as f32;
                // println!( "GOING NEXT ROUND: next_x {} next_y {}", next_x, next_y);
            }
        }
        let distance = distance(player_x, player_y, next_x, next_y);
        // println!( "GOING OUT with distance {}", distance);
        return Ray::new(angle, distance, false);
    }
}

pub fn distance(x1: f32, y1:f32, x2:f32, y2:f32) -> f32 {
    ((x2-x1).powi(2) + (y2-y1).powi(2)).sqrt()
}

pub fn to_radians(deg: f32) -> f32 {
    (deg * PI as f32) / 180.0
}
