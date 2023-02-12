use std::{net::SocketAddr, collections::HashMap};
use macroquad::{prelude::{GRAY, BLACK}, shapes::draw_rectangle, text::draw_text};

use crate::{common::constants::BOX_SIZE, Player, GameWindow};

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
        for (_, player) in players.clone() {
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
