use macroquad::prelude::*;
use std::collections::HashMap;

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

const CAP_SIZE: f32 = 1.0;
const PADDING: f32 = 20.0;
const PLAYER_SIZE:f32 = 10.0;
const PLAYER_RAY_LENGTH:f32 = PLAYER_SIZE * 2.0;

#[derive(Debug, Clone)]
pub struct GameBoard {
    pub board: Vec<Vec<i32>>,
    pub setup: Settings,
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub center_x_offset: f32,
    pub center_y_offset: f32,
    pub cube_size: f32,
    pub board_height: f32,
}

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub speed: f32,
}

impl Player {
    pub fn new(x:f32, y:f32, angle:f32, speed:f32) -> Player {
        Self {x, y, angle, speed}    
    }
    pub fn draw(&self) {
        // player.x = (j as f32 * self.setup.cube_size) + PADDING;
        // player.y = (i as f32 * self.setup.cube_size) + self.setup.center_y_offset*2.0 - PADDING;
        
        //draw player on the screen
        draw_circle(self.x, self.y, PLAYER_SIZE/2.0, RED);
        
        //Draw a line from player to show it`s direction
        draw_line(self.x, self.y, self.x + self.angle.cos() * PLAYER_RAY_LENGTH, self.y + self.angle.sin() * PLAYER_RAY_LENGTH, 1.0, RED);
        
        //Draw rays from player
        // for ray in rays.iter() {
        //     draw_line(player.x, player.y + PLAYER_SIZE/2.0, (player.x + ray.angle.cos() * ray.distance), (player.y + ray.angle.sin() * ray.distance)+ PLAYER_SIZE/2.0, 1.0, YELLOW);
        // }
    }
}

impl Settings{
    pub fn new(board: Vec<Vec<i32>>)-> Self{
        let horizontal_cubes = board[0].len() as f32;
        let cube_size = get_cube_size(horizontal_cubes);

        Self { 
            center_x_offset: get_center_x(horizontal_cubes, cube_size),
            center_y_offset: get_center_y(board.len() as f32, cube_size),
            cube_size,
            board_height: horizontal_cubes,
        }
    }
}

impl GameBoard {
    pub fn new(map: Vec<Vec<i32>>) -> Self {
        let setup = Settings::new(map.clone());
        Self {
            board: map.clone(),
            setup
        }
    }

    pub fn draw(&self) {
        draw_rectangle(
            20.0,
            self.setup.center_y_offset + self.setup.center_y_offset - CAP_SIZE - PADDING,
            (self.board[0].len() as f32 * self.setup.cube_size) + CAP_SIZE,
            (self.board.len() as f32 * self.setup.cube_size) + CAP_SIZE,
            WHITE,
        );
        for i in 0..self.board.len() {
            for j in 0..self.board[0].len() {
                match self.board[i][j] {
                    1 => {
                        draw_rectangle(
                            (j as f32 * self.setup.cube_size) + PADDING,
                            (i as f32 * self.setup.cube_size) + self.setup.center_y_offset*2.0 - PADDING,
                            self.setup.cube_size - CAP_SIZE,
                            self.setup.cube_size - CAP_SIZE,
                            self.match_color(self.board[i][j]),
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
}

fn get_center_x(horizontal_cubes: f32, cube_size: f32) -> f32 {
    return (screen_width() - (horizontal_cubes as f32 * cube_size)) /2.0;
}

fn get_center_y(lines: f32, cube_size: f32) -> f32 {
    return (screen_height() - (lines as f32 * cube_size)) /2.0;
}

fn get_cube_size(horizontal_cubes: f32) -> f32 {
    if horizontal_cubes <= 10.0 {return 30.0} 
    if horizontal_cubes <= 20.0 {return 20.0} 
    if horizontal_cubes <= 30.0 {return 10.0} 
    return 10.0
}


#[derive(Debug, Clone)]
pub struct MazeVisual {
    pub width: f32,
    pub height: f32,
}

impl MazeVisual {
    pub fn new(game_board: GameBoard) -> Self {
        Self {
            width: screen_width() - PADDING * 2.0,
            height: screen_height() - PADDING * 3.0 - (game_board.board.len() as f32 * game_board.setup.cube_size) + CAP_SIZE,
        }
    }
    pub fn draw(&self) {
        draw_rectangle(
            PADDING,
            PADDING,
            self.width,
            self.height,
            WHITE,
        );
    }
}

#[derive(Debug, Clone)]
pub struct ScoreBoard {
    pub width: f32,
    pub height: f32,
    pub text_address_x: f32,
    pub text_address_y: f32,
    pub players: HashMap<String, String>,
}

impl ScoreBoard{
    pub fn new(visual_box: MazeVisual, players: HashMap<String, String>) -> Self {
        Self {
            width: screen_width() / 2.0 - PADDING * 5.0 ,
            height: screen_height() - PADDING * 3.0 - visual_box.height,
            text_address_x: screen_width() / 2.0,
            text_address_y: PADDING * 3.0 + visual_box.height,
            players
        }
    }
    pub fn draw(&self) {
        draw_text("SCORE:", self.text_address_x, self.text_address_y, 20.0, RED);
        draw_rectangle(
            screen_width() / 2.0 + PADDING * 4.0,
            screen_height() - self.height - PADDING,
            self.width,
            self.height,
            GRAY,
        );


        let mut y_addition = 0.0;

        let mut hash_vec: Vec<(&String, &String)> = self.players.iter().collect();
        hash_vec.sort_by(|a, b| b.1.cmp(a.1));
        for (player, score) in hash_vec.iter() {
            draw_text(&player.clone(), self.text_address_x + 90.0, self.text_address_y + y_addition, 20.0, BLACK);
            draw_text(score, self.text_address_x + 200.0, self.text_address_y + y_addition, 20.0, BLACK);
            y_addition += 20.0;
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
