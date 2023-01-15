use crate::maze::{Grid, HIGH, LOW, MEDIUM};
use macroquad::prelude::*;
use crate::map::WALL;
use crate::player::*;

mod maze;
mod map;
mod player;

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
    let spawn_point  = Point::new(100.0,100.0);
    let mut player = Player::new(spawn_point);

    let mut grid = Grid::new(10, 10, LOW);
    grid.generate_maze();
    let map = grid.convert_to_map();
    loop {
        //
        map.draw();
        grid.draw();

        // Player::draw();
        player.draw();




        next_frame().await
    }
}