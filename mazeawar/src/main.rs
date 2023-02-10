// #![allow(dead_code)]
// #![allow(unused_imports)]
// #![allow(unused_variables)]

use std::fs::read;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::process::exit;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread;
use local_ip_address::local_ip;
use macroquad::prelude::*;
use map::Map;
use serde_json::*;
use regex::Regex;
use std::collections::HashMap;
use macroquad::time::get_fps;

use crate::client_server::*;
use crate::player::*;
use crate::utils::*;
use crate::map::GameWindow;
use crate::ray::Ray;

mod client_server;
mod map;
mod maze;
mod player;
mod utils;
mod ray;

fn window_conf() -> Conf {
    Conf {
        window_title: "MAZE".to_owned(),
        window_height: 850,
        window_width: 1220,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> std::io::Result<()> {
    //option for prod
    //add user input for server ip and user name
    // let input_ip = input::read("Enter IP address: ".to_string(), input::InputType::Ip);
    // let server_addr = convert::to_ip(input_ip);
    // let user_name = input::read("Enter Name:  ".to_string(), input::InputType::Name);

    //option for tests
    //to test this has to be changed to local ip address
    // let server_addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192,168, 1, 174)), 4242);
    let my_local_ip = local_ip().unwrap();
    let server_addr: SocketAddr = SocketAddr::new(my_local_ip, 4242);

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
    
    let zero_point = Point::zero();
    let mut data = Data::default();
    let mut is_shot = false;
    let mut shooting_timer = 0;
    // Current display updates based on events, should be from back
    loop {
        if let Ok(received_data) = rx.try_recv() {
            data = received_data;
        }
        
        let game_window: GameWindow = data.map.draw(&data.players);
        let mut me = Player::new(zero_point);
        let mut enemy_positions: Vec<Point> = vec![];
        //FIRST FOUND ME IN THE LIST to settle the position
        for (src, player) in &data.players {
            if src.to_string() == sender_clone.get_address().to_string() {
                me = player.clone();
                me.draw(game_window.clone(), data.map.clone(), is_shot);
            }
        }

        // THEN DRAW ONLY THE ENEMIES
        for (src, player) in &data.players {
            if src.to_string() != sender_clone.get_address().to_string() {
                let visible: bool = data.map.check_visibility(&me, &player); 
                me.draw_enemy(player.clone(), &game_window, visible);
                enemy_positions.push(player.location);
            }
        }


        //  IT IS UGLY FIX :D 
        if shooting_timer <= 0 {
            is_shot = false;
        }else{
            me.shoot(data.map.0.clone());
            shooting_timer -= 1;
        }
        if is_key_pressed(KeyCode::Space) {
            is_shot = true;
            shooting_timer = 20;
            sender_clone.send_message("shoot", json!(me));
        }
        listen_move_events(&sender_clone, me, &data.map, &enemy_positions);
        if is_key_pressed(KeyCode::Escape) {
            sender_clone.send_message("I QUIT", json!(""));
            exit(1)
        }
        draw_text(&format!("FPS: {}", get_fps()), screen_width() - 200.0, 30.0, 25.0, BLACK);
        next_frame().await;
    }
}

pub fn listen_move_events(client: &Client, mut me: Player, map: &Map, enemy_positions: &Vec<Point>) {
    let mut action:bool = false;
    if is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left) {
        me.turn_left();
        action = true;
    }
    if is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right) {
        me.turn_right();
        action = true;
    }
    if is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up) {
        action = me.step(map, enemy_positions);
    }
    if is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down) {
        action = me.step(map, enemy_positions);
    }
    if action {
        client.send_message("movement", json!(me))
    }
}
