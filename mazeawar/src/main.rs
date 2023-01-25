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
    let (input_ip, user_name) = get_user_data();
    let (addr, port) = parse_ip(input_ip);

    let server_addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(addr[0], addr[1], addr[2], addr[3])), port);
    let client = Client::new(server_addr);
    let sender_clone = Arc::new(client);
    let receiver_clone = sender_clone.clone();
    let (tx, rx) = channel::<Data>();

    thread::spawn(move || {
        receiver_clone.send_message("connect", json!(""));
        loop{
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
            if src.to_string() == sender_clone.get_address().to_string(){
                my_point = player.location
            }
            player.draw();
        }

        listen_move_events(&my_point, &sender_clone);
        if is_key_pressed(KeyCode::Escape) {
            sender_clone.send_message("I QUIT",  json!(""));
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

fn get_user_data()-> (String, String) {
    use std::io::{stdin, stdout, Write};
    let ip_re = Regex::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:\d+").unwrap();
    let mut ip = String::new();
    let mut message = "Enter IP Address: ";

    if !ip_re.is_match(&*ip) {
        loop {
            ip = String::new();
            print!("{}", message);
            let _ = stdout().flush();
            stdin().read_line(&mut ip).expect("Did not enter a correct string");
            if let Some('\n') = ip.chars().next_back() {
                ip.pop();
            }
            if ip_re.is_match(&*ip) {
                break;
            }
            message = "Entered IP is incorrect. Try again: ";
        }
    }

    let mut name = String::new();
    print!("Enter Name: ");
    let _ = stdout().flush();
    stdin().read_line(&mut name).expect("Did not enter a correct string");
    if let Some('\n') = name.chars().next_back() {
        name.pop();
    }
    return (ip, name);
}

fn parse_ip(ip: String) ->(Vec<u8>, u16) {
    let re: Regex = Regex::new(r"(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3}):(\d+)").unwrap();
    let captures = re.captures(&*ip).unwrap();
    let mut res:Vec<u8> = Vec::new();
    for i in 1..5 {
        let nb = captures.get(i).map(|a| a.as_str().parse::<u8>().unwrap());
        res.push(nb.unwrap());
    }
    let port = (captures.get(5).map(|port| port.as_str().parse::<u16>().unwrap())).unwrap();
    return (res, port);
}
