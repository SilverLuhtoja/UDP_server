use crate::maze::{Grid, HIGH, LOW, MEDIUM};
use macroquad::prelude::*;
use crate::map::WALL;


mod maze;
mod map;


fn window_conf() -> Conf {
    Conf {
        window_title: "MAZE".to_owned(),
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut grid = Grid::new(5, 5, HIGH);
    grid.generate_maze();
    let map = grid.convert_to_map();
    loop {
        map.draw();
        grid.draw();
        next_frame().await
    }
}