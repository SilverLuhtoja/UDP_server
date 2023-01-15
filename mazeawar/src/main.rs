use macroquad::prelude::*;
use std::process::exit;
use crate::maze::{Grid, LOW};
use crate::player::*;
use crate::host_server::*;
use local_ip_address::local_ip;
use std::net::UdpSocket;

mod maze;
mod map;
mod player;
mod host_server;

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
    // Currently trying to just kind of connect two players and update their positions in real time
    // this means that data have to come from server --> to update screen list Of players draw, playersss have to send data to server
    // Implement only single connection, send movement data and draw when received answer from hostServer
    let fake_connection = "10.5.0.2:4242";
    let mut host_socket= Host::new(local_ip().unwrap());
    
    
    let spawn_point  = Point::new(100.0,100.0);
    let mut player = Player::new(spawn_point);
    let mut grid = Grid::new(10, 10, LOW);
    grid.generate_maze();
    let map = grid.convert_to_map();
    
    host_socket.add_player(player);


    println!("Creating server : {:?}.Listening....", host_socket);
    println!("Waiting messages:");
    host_socket.socket.send_to("connect".as_bytes(),&fake_connection).unwrap();
    loop {
        // host_socket.read_incoming_messages();
        map.draw();
        grid.draw();

        player.draw(); // allPlayers update (including player itself)
        player.update_movement(); // send data to server
        
        if is_key_pressed(KeyCode::Escape) { exit(1) } // if exit, shut down connection, update clients list
        next_frame().await
    }
}