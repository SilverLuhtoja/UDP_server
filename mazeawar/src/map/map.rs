use serde::{Serialize, Deserialize};
use macroquad::prelude::*;
use std::net::SocketAddr;
use std::collections::HashMap;

use super::game_window::GameWindow;
use super::score_board::ScoreBoard;

use crate::common::constants::BOX_SIZE;
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
    
    pub fn out_of_map_bounce(&self, x: f32, y: f32) ->bool {
        x < 0.0 || x >= self.width() as f32 || y < 0.0 || y >= self.height() as f32
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
