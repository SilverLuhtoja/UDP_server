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
use regex::Regex;

use crate::client_server::*;
use crate::player::*;

mod client_server;
mod map;
mod maze;
mod player;
mod utils;

fn window_conf() -> Conf {
    Conf {
        window_title: "MAZE".to_owned(),
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}

pub enum InputType {
    Ip,
    Name,
}

#[macroquad::main(window_conf)]
async fn main() -> std::io::Result<()> {
    //add user input for server ip and user name
    // let input_ip = get_user_input("Enter IP address: ".to_string(), InputType::Ip);
    // let server_addr = utils::convert::to_ip(input_ip);
    // let user_name = get_user_input("Enter Name:  ".to_string(), InputType::Name);

    //to test this has to be changed to local ip address
    let server_addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192,168, 0, 57)), 4242);

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
    // Current display updates based on events, should be from back
    loop {
        if let Ok(received_data) = rx.try_recv() {
            data = received_data;
        }
        data.map.draw();
        for (src, player) in &data.players {
            if src.to_string() == sender_clone.get_address().to_string() {
                my_point = player.location
            }
            player.draw();
        }

        listen_move_events(&my_point, &sender_clone);
        if is_key_pressed(KeyCode::Escape) {
            sender_clone.send_message("I QUIT", json!(""));
            exit(1)
        }
        next_frame().await;
    }
}

pub fn listen_move_events(my_point: &Point, client: &Client) {
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
    if point.is_moved() {
        client.send_message("movement", json!(point))
    }
}

fn get_user_input(mut message: String, input_type: InputType) -> String {
    use std::io::{stdin, stdout, Write};
    let mut input = String::new();
    loop {
        input = String::new();
        print!("{}", message);
        let _ = stdout().flush();
        stdin().read_line(&mut input).expect("Did not enter a correct string");
        if let Some('\n') = input.chars().next_back() {
            input.pop();
        }
        match input_type {
            InputType::Ip => {
                if utils::validate::ip(input.clone()) { break; }
                message = "Entered IP is incorrect. Try again: ".to_string();
            }
            InputType::Name => {
                if utils::validate::user_name(input.clone()) { break; }
                message = "Entered name is too short. Try again: ".to_string();
            }
        }
    }
    return input;
}
