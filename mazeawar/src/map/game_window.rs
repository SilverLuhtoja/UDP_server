use macroquad::window::{screen_height, screen_width};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize,Default)]
/*Used to set-up visual part*/
pub struct GameWindow{
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
    pub fn new() -> Self {
        Self { 
            minimap_start_x: 0.0,
            minimap_start_y: 0.0,
            minimap_finish_x: 0.0,
            minimap_finish_y: 0.0,
    
            score_board_start_x: 0.0,
            score_board_start_y: 0.0,
            score_board_finish_x: 0.0,
            score_board_finish_y: screen_height(),

            visual_window_start_x: 0.0,
            visual_window_start_y: 0.0,
            visual_window_finish_x: screen_width(),
            visual_window_finish_y: screen_height(),
        }
    }

    
    pub fn get_visual_screen_center_point(&self) -> (f32,f32){
        let visual_screen_dimensions = self.get_visual_screen_width_height();
        let visual_screen_starting_points = self.get_visual_screen_starting_point();
        let visual_screen_center_points = (visual_screen_dimensions.0 / 2.0, visual_screen_dimensions.1 / 2.0);
        (visual_screen_starting_points.0 + visual_screen_center_points.0, visual_screen_starting_points.1 + visual_screen_center_points.1)
    }

    pub fn get_visual_screen_width_height(&self)-> (f32,f32) {
        let width = self.visual_window_finish_x - self.visual_window_start_x;
        let height= self.visual_window_finish_y - self.visual_window_start_y;
        (width,height)
    }

    pub fn get_visual_screen_starting_point(&self)-> (f32,f32) {
        (self.visual_window_start_x,self.visual_window_start_y)
    }
}


