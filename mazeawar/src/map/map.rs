
use serde::{Serialize, Deserialize};
use macroquad::prelude::*;
use std::net::SocketAddr;
use std::collections::HashMap;

use crate::common::constants::BOX_SIZE;
use crate::player;
use crate::player::player::{Player, Direction};

pub const FLOOR: i32 = 0;
pub const WALL: i32 = 1;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize,Default)]
pub struct Map(
    pub Vec<Vec<i32>>,
);

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Self(vec![vec![FLOOR; width]; height])
    }
    pub fn width(&self) -> usize {
        self.0[0].len()
    }
    pub fn height(&self) -> usize {
        self.0.len()
    }
    pub fn draw(&self, players: &HashMap<SocketAddr, Player>) -> GameWindow {
        let mut game_window: GameWindow = GameWindow::new();
        let offset: f32 = 0.0;
        let size: f32 = BOX_SIZE;
        //Draw minimap
        for i in 0..self.height() {
            for j in 0..self.width() {
                if self.0[i][j] == WALL {
                    draw_rectangle(j as f32 * size + offset, i as f32 * size + offset, size, size, WHITE);
                } else {
                    draw_rectangle(j as f32 * size + offset, i as f32 * size + offset, size, size, BLACK);
                }
                if i == self.height()-1 && j == self.width()-1 {
                    game_window.minimap_finish_x = j as f32 * size + offset;
                    game_window.minimap_finish_y = i as f32 * size + offset
                }
            }
        }
        game_window.visual_window_start_x = game_window.minimap_finish_x + BOX_SIZE;
        game_window.score_board_start_y = game_window.minimap_finish_y + BOX_SIZE;
        game_window.score_board_finish_x = game_window.minimap_finish_x + BOX_SIZE;

        //draw scoreboard
        let score_board = ScoreBoard::new(game_window.clone(), players.clone());
        score_board.draw();
        return game_window;
    }

    pub fn out_of_map_bounce(&self, x: f32, y: f32) ->bool {
        x < 0.0 || x >= self.width() as f32 || y < 0.0 || y >= self.height() as f32
    }

    pub fn is_wall(&self, row: f32, column: f32) -> bool{
        self.0[row as usize][column as usize] == 1
    }

    pub fn check_visibility(&self, player1: &Player, player2: &Player) -> bool {
        let mut pl1 = (player1.location.x / BOX_SIZE, player1.location.y / BOX_SIZE);
        let pl2 = (player2.location.x / BOX_SIZE, player2.location.y / BOX_SIZE);
        let difference = looking_direction_calculation_difference(player1.looking_at);
        while pl1 != pl2{
            if self.is_wall(pl1.1, pl1.0){
                return false
            }
            pl1 = add_difference(pl1, difference)
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




#[derive(Debug, Clone)]
pub struct ScoreBoard{
    pub start_x: f32,
    pub start_y: f32,
    pub width: f32,
    pub height: f32,
    pub players: Vec<Player>,
}

impl ScoreBoard{
    pub fn new(game_window: GameWindow, players: HashMap<SocketAddr, Player>) -> Self {
        
        //sort players by highest score
        let mut sorted: Vec<Player> = vec![];
        for (_, player) in players {
            sorted.push(player);
        }

        sorted.sort_by(|a, b| b.score.cmp(&a.score));

        Self{
            start_x: 0.0,
            start_y: game_window.score_board_start_y,
            width: game_window.score_board_finish_x - 0.0,
            height: game_window.score_board_finish_y - game_window.score_board_start_y,
            players: sorted,
        }
    }
    pub fn draw(&self) {
        draw_rectangle(
            self.start_x,
            self.start_y,
            self.width,
            self.height,
            GRAY,
        );
        let mut y_addition = BOX_SIZE;
        for player in &self.players {
            draw_text(&player.username.clone(), self.start_x + BOX_SIZE, self.start_y + y_addition, BOX_SIZE, BLACK); //name
            draw_text(&player.score.to_string(), self.start_x + BOX_SIZE * 10.0, self.start_y + y_addition, BOX_SIZE, BLACK); //score
            y_addition += BOX_SIZE;
        }
    
    }
}
