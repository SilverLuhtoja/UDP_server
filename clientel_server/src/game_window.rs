// #[allow(unused_assignments)]
use macroquad::prelude::*;
use macroquad::rand::gen_range;
use std::collections::HashMap;

pub const PADDING: f32 = 20.0;

#[derive(Debug, Clone)]
pub struct GameWindow{
    pub minimap: Vec<Vec<i32>>,
    pub minimap_cube_size: f32,
    pub minimap_start_x: f32,
    pub minimap_start_y:f32,
    pub minimap_finish_x: f32,
    pub minimap_finish_y:f32,

    pub visual_window_start_x: f32,
    pub visual_window_start_y:f32,
    pub visual_window_finish_x: f32,
    pub visual_window_finish_y:f32,

    pub score_board_start_x: f32,
    pub score_board_start_y:f32,
    pub score_board_finish_x: f32,
    pub score_board_finish_y:f32,
}

impl GameWindow {
    pub fn new(board: Vec<Vec<i32>>) -> Self {

        let horizontal_cubes = board[0].len() as f32;
        let mut cube_size:f32 = get_cube_size(horizontal_cubes);

        let visual_width:f32 = screen_width() - PADDING * 2.0;
        let visual_height:f32 = screen_height() - PADDING * 3.0 - (board.len() as f32 * cube_size);

        Self { 
            minimap: board.clone(),
            minimap_cube_size: cube_size,
            visual_window_start_x: PADDING,
            visual_window_start_y: PADDING,
            visual_window_finish_x: PADDING + visual_width,
            visual_window_finish_y: PADDING + visual_height,
    
            minimap_start_x: PADDING,
            minimap_start_y: PADDING * 2.0 + visual_height,
            minimap_finish_x: screen_width() - PADDING,
            minimap_finish_y: screen_height() - PADDING,
    
            score_board_start_x: (screen_width() / 4.0)*3.0,
            score_board_start_y: PADDING * 2.0 + visual_height,
            score_board_finish_x: screen_width() - PADDING,
            score_board_finish_y: screen_height() - PADDING,
        }
    }

    pub fn draw(&self){
        draw_rectangle(
            self.minimap_start_x,
            self.minimap_start_y,
            self.minimap_finish_x / 4.0 - self.minimap_start_x,
            self.minimap_finish_y - self.minimap_start_y,
            WHITE,
        );

        for i in 0..self.minimap.len() {
            for j in 0..self.minimap[0].len() {
                match self.minimap[i][j] {
                    1 => {
                        // self.wall_coordinates.push(((j as f32 * self.setup.cube_size) + PADDING, (i as f32 * self.setup.cube_size) + self.setup.center_y_offset*2.0 - PADDING));
                        draw_rectangle(
                            (j as f32 * self.minimap_cube_size) + self.minimap_start_x,
                            (i as f32 * self.minimap_cube_size) + self.minimap_start_y,
                            self.minimap_cube_size,
                            self.minimap_cube_size,
                            self.match_color(self.minimap[i][j]),
                        );
                    },
                    _=> (),
                }
            }
        }
    }
    fn match_color(&self, ch: i32) -> Color {
        match ch {
            1  => GRAY,
            0  => WHITE,
            _ => GREEN,
        }
    }
    pub fn out_of_map_bounce(&self, x: f32, y: f32) ->bool {
        x < 0.0 || x >= self.minimap[0].len() as f32 || y < 0.0 || y >= self.minimap.len() as f32
    }

    pub fn get_random_empty_space(&self)-> (usize, usize){
        let mut x:usize = 0;
        let mut y:usize = 0;
        while self.minimap[y][x] != 0 {
            y = gen_range(0, self.minimap.len());
            x = gen_range(0, self.minimap[0].len());
            println!("X {}", x);
            println!("Y {}", y);
        }
        return (x, y)
    }
}


fn get_cube_size(horizontal_cubes: f32) -> f32 {
    println!("kast {}", horizontal_cubes);
    if horizontal_cubes <= 10.0 {return 30.0} 
    if horizontal_cubes <= 20.0 {return 20.0} 
    if horizontal_cubes <= 30.0 {return 15.0} 
    return 10.0
}
