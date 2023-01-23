// #![allow(dead_code)]
// #![allow(unused_imports)]
// #![allow(unused_variables)]

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::process::exit;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread;
use macroquad::prelude::*;
use serde_json::*;

use crate::client_server::*;
use crate::player::*;

mod client_server;
mod map;
mod maze;
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
async fn main() -> std::io::Result<()> {
    let server_addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 5, 0, 2)), 4242);
    let client = Client::new(server_addr);

    let sender_clone = Arc::new(client);
    let receiver_clone = sender_clone.clone();
    let (tx, rx) = channel::<Data>();

    thread::spawn(move ||{
        receiver_clone.send_message("connect", json!(""));
        loop{

            let data = receiver_clone.read_message();
            tx.send(data).unwrap()
        }
    });
    
    let data = rx.recv().unwrap();
    let mut my_point = Point::zero();
    loop {
        // client.send_message("update", json!(""));
        listen_move_events(&my_point, &sender_clone);
        data.map.draw();
        for (src, player) in &data.players {
            if src.to_string() == sender_clone.get_address().to_string(){
                my_point = player.location
            }
            player.draw();
        }

        if is_key_pressed(KeyCode::Escape) {
            exit(1)
        }
        next_frame().await;
    }
}

pub fn listen_move_events(my_point: &Point, client: &Client){
    let mut point = Point::zero();
    if is_key_pressed(KeyCode::A) {
        point = Point::new(my_point.x - 20.0, my_point.y);
    }
    if is_key_pressed(KeyCode::D) {
        point = Point::new(my_point.x + 20.0, my_point.y);
    }
    if is_key_pressed(KeyCode::W) {
        point = Point::new(my_point.x, my_point.y - 20.0);
    }
    if is_key_pressed(KeyCode::S) {
        point = Point::new(my_point.x, my_point.y + 20.0);
    }
    if point.is_moved(){
        client.send_message("movement", json!(point))
    }
}
