// #![allow(dead_code)]
// #![allow(unused_imports)]
// #![allow(unused_variables)]

use std::fs::read;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::process::exit;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread;
use macroquad::prelude::*;
use serde_json::*;
use regex::Regex;
use std::collections::HashMap;
use macroquad::time::get_fps;

use crate::client_server::*;
use crate::player::*;
use crate::utils::*;
use crate::map::GameWindow;

mod client_server;
mod map;
mod maze;
mod player;
mod utils;
mod ray;

fn window_conf() -> Conf {
    Conf {
        window_title: "MAZE".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> std::io::Result<()> {
    //option for prod
    //add user input for server ip and user name
    let input_ip = input::read("Enter IP address: ".to_string(), input::InputType::Ip);
    let server_addr = convert::to_ip(input_ip);
    let user_name = input::read("Enter Name:  ".to_string(), input::InputType::Name);

    //option for tests
    //to test this has to be changed to local ip address
    // let server_addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192,168, 0, 43)), 4242);

    let client = Client::new(server_addr);
    let sender_clone = Arc::new(client);
    let receiver_clone = sender_clone.clone();
    let (tx, rx) = channel::<Data>();

    thread::spawn(move || {
        receiver_clone.send_message("connect", json!(""));
        loop {
            let received_data = receiver_clone.read_message();
            tx.send(received_data).unwrap()
        }
    });

    let mut data = Data::default();
    let mut my_point = Point::zero();
    let mut is_shot = false;
    // Current display updates based on events, should be from back
    loop {
        if let Ok(received_data) = rx.try_recv() {
            data = received_data;
        }
        let game_window: GameWindow = data.map.draw(&data.players);
        let mut me = Player::new(my_point);
        for (src, player) in &data.players {
            if src.to_string() == sender_clone.get_address().to_string() {
                me = player.clone();
                player.draw(true, game_window.clone(), data.map.clone(), is_shot);
            } else{
                player.draw(false, game_window.clone(), data.map.clone(), false);
            }
        }
        is_shot = false;
        if is_key_down(KeyCode::Space) {
            me.shoot(data.map.0.clone());
            is_shot = true;
            &sender_clone.send_message("shoot", json!(me));
        }
        listen_move_events(&sender_clone, me, data.map.0.clone());
        if is_key_pressed(KeyCode::Escape) {
            sender_clone.send_message("I QUIT", json!(""));
            exit(1)
        }
        draw_text(&format!("FPS: {}", get_fps()), screen_width() - 200.0, 30.0, 25.0, BLACK);
        next_frame().await;
    }
}

pub fn listen_move_events(client: &Client, mut me: Player, map: Vec<Vec<i32>>) {
    let mut action:bool = false;
    if is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left) {
        me.turn_left();
        action = true;
    }
    if is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right) {
        me.turn_right();
        action = true;
    }
    if is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up){
        me.step(20.0, map.clone());
        action = true;
    }
    if is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down){
        me.step(-20.0, map.clone());
        action = true;
    }
    if action {
        client.send_message("movement", json!(me))
    }
}
