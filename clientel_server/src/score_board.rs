
use macroquad::prelude::*;
use macroquad::rand::gen_range;
use std::collections::HashMap;

use crate::game_window::*;

#[derive(Debug, Clone)]
pub struct ScoreBoard {
    pub start_x: f32,
    pub start_y: f32,
    pub width: f32,
    pub height: f32,
    pub text_address_x: f32,
    pub text_address_y: f32,
    pub players: HashMap<String, String>,
}

impl ScoreBoard{
    pub fn new(game_window: GameWindow, players: HashMap<String, String>) -> Self {
        Self {
            start_x: game_window.score_board_start_x,
            start_y: game_window.score_board_start_y,
            width: game_window.score_board_finish_x - game_window.score_board_start_x,
            height: game_window.score_board_finish_y - game_window.score_board_start_y,
            text_address_x: game_window.score_board_start_x,
            text_address_y: PADDING * 2.0 + game_window.visual_window_finish_y,
            players
        }
    }
    pub fn draw(&self) {
        //TODO -> If players are more then 10 -> all scores to be seen -> TODO!!!!
        draw_rectangle(
            self.start_x,
            self.start_y,
            self.width,
            self.height,
            GRAY,
        );
        let mut y_addition = 0.0;

        let mut hash_vec: Vec<(&String, &String)> = self.players.iter().collect();
        hash_vec.sort_by(|a, b| b.1.cmp(a.1)); //TODO -> NOT CORRECT!!!
        for (player, score) in hash_vec.iter() {
            if screen_width() < 600.0 {
                draw_text(&player.clone(), self.text_address_x + PADDING, self.text_address_y + y_addition, 20.0, BLACK); //name
                y_addition += 20.0;
                draw_text(score, self.text_address_x + PADDING, self.text_address_y + y_addition, 20.0, BLACK); //score
                y_addition += 20.0;
            } else {
                draw_text(&player.clone(), self.text_address_x + PADDING, self.text_address_y + y_addition, 20.0, BLACK); //name
                draw_text(score, self.text_address_x + PADDING * 5.0, self.text_address_y + y_addition, 20.0, BLACK); //score
                y_addition += 20.0;
            }
        }
    }
}
