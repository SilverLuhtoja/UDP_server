use macroquad::prelude::*;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

const CAP_SIZE: f32 = 1.0;
const PADDING: f32 = 20.0;


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
            self.setup.center_y_offset + self.setup.center_y_offset - CAP_SIZE - 20.0,
            (self.board[0].len() as f32 * self.setup.cube_size) + CAP_SIZE,
            (self.board.len() as f32 * self.setup.cube_size) + CAP_SIZE,
            WHITE,
        );
        for i in 0..self.board.len() {
            for j in 0..self.board[0].len() {
                draw_rectangle(
                    (j as f32 * self.setup.cube_size) + 20.0,
                    (i as f32 * self.setup.cube_size) + self.setup.center_y_offset + self.setup.center_y_offset - 20.0,
                    self.setup.cube_size - CAP_SIZE,
                    self.setup.cube_size - CAP_SIZE,
                    self.match_color(self.board[i][j]),
                );
            }
        }
    }

    fn match_color(&self, ch: i32) -> Color {
        match ch {
            1  => GRAY,
            0  => WHITE,
            _ => RED,
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
    if horizontal_cubes <= 10.0 {return 20.0} 
    if horizontal_cubes <= 20.0 {return 10.0} 
    if horizontal_cubes <= 30.0 {return 5.0} 
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
}

impl ScoreBoard {
    pub fn new(visual_box: MazeVisual) -> Self {
        Self {
            width: screen_width() / 2.0 - PADDING * 5.0 ,
            height: screen_height() - PADDING * 3.0 - visual_box.height,
            text_address_x: screen_width() / 2.0,
            text_address_y: PADDING * 3.0 + visual_box.height,
        }
    }
    pub fn draw(&self) {
        draw_text("SCORE:", self.text_address_x, self.text_address_y, 20.0, RED);
        draw_rectangle(
            screen_width() / 2.0 + PADDING * 4.0,
            screen_height() - self.height - PADDING,
            self.width,
            self.height,
            YELLOW,
        );
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
