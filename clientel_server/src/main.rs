use std::io::{self, BufRead};
use std::net::UdpSocket;
use std::str;
use macroquad::prelude::*;
use std::collections::HashMap;
use std::f64::consts::PI;


use clientel_server::player::Player;
use clientel_server::game_window::GameWindow;
use clientel_server::score_board::ScoreBoard;



const ADDR:&str = "127.0.0.1";
const SERVER_PORT:u16 = 4242;
const CLIENT_PORT:u16 = 34254;

// static map: Vec<u8>;

fn window_conf() -> Conf {
    Conf {
        window_title: "MAZE WARS".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind(format!("{}:{}",ADDR, CLIENT_PORT))?;
    println!("Establishing UDP socket address at : {:?}", socket);
    try_to_connect(&socket); // "fake handshake"
    let map: Vec<Vec<i32>> = vec![
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1],
        vec![1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1],
        vec![1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1],
        vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1],
        vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1],
        vec![1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1],
        vec![1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1],
        vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1],
        vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];
    // let map: Vec<Vec<i32>> = vec![
    //     vec![1, 1, 1, 1, 1, 1, 1, 1],
    //     vec![1, 0, 0, 0, 0, 0, 0, 1],
    //     vec![1, 0, 1, 1, 0, 1, 1, 1],
    //     vec![1, 0, 0, 0, 0, 0, 1, 1],
    //     vec![1, 0, 1, 0, 1, 0, 0, 1],
    //     vec![1, 0, 1, 0, 1, 0, 1, 1],
    //     vec![1, 0, 0, 0, 0, 0, 0, 1],
    //     vec![1, 0, 1, 1, 0, 1, 1, 1],
    //     vec![1, 0, 0, 0, 0, 0, 1, 1],
    //     vec![1, 0, 1, 0, 1, 0, 1, 1],
    //     vec![1, 0, 1, 0, 1, 0, 1, 1],
    //     vec![1, 1, 1, 1, 1, 1, 1, 1],
    // ];

    // let map: Vec<Vec<i32>> = vec![
    //     vec![1, 1, 1, 1, 1],
    //     vec![1, 0, 1, 0, 1],
    //     vec![1, 0, 0, 0, 1],
    //     vec![1, 0, 1, 0, 1],
    //     vec![1, 1, 1, 1, 1],
    // ];
  
    
    let mut game_window = GameWindow::new(map.clone());
    let mut player = Player::new(game_window.minimap_cube_size);
    player.set_position(game_window.clone());

    let mut players = HashMap::new();
    players.insert(String::from("Anna"), String::from("130"));
    players.insert(String::from("Silver"), String::from("20"));
    players.insert(String::from("Valeria"), String::from("10"));
    players.insert(String::from("Emil"), String::from("05"));

    let score_board = ScoreBoard::new(game_window.clone(), players.clone());

    loop{

        clear_background(Color::new(0.0, 0.0, 0.0, 0.8));


        if is_key_down(KeyCode::Left) {
            println!("\nwrite to server:");
            player.angle -= 0.01;
            if player.angle < 0.0 {
                player.angle += (2.0 * PI) as f32;
            }
            socket.send("Left".as_bytes()).expect("Error on send");
        }

        if is_key_down(KeyCode::Right) {
            println!("\nwrite to server:");
            player.angle += 0.01;
            if player.angle > (2.0 * PI) as f32 {
                player.angle -= (2.0 * PI) as f32;
            }
            socket.send("Right".as_bytes()).expect("Error on send");
        }

        if is_key_down(KeyCode::Up) {
            println!("\nwrite to server:");
            player.speed = 0.5;
            player.step();
            socket.send("Up".as_bytes()).expect("Error on send");
        }

        if is_key_down(KeyCode::Down) {
            println!("\nwrite to server:");
            player.speed = -0.5;
            player.step();
            socket.send("Down".as_bytes()).expect("Error on send");
        }

        if is_key_pressed(KeyCode::Space) {
            println!("\nwrite to server:");
            socket.send("Shoot".as_bytes()).expect("Error on send");
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        game_window.draw();
        score_board.draw();
        player.draw(game_window.clone());

        next_frame().await
    }

    
    Ok(())
}

// this is basically UDP handshake, to see if anything is even otherside to receive messages
fn try_to_connect(socket : &UdpSocket){
    let addr = &format!("{}:{}",ADDR, SERVER_PORT);
    socket.connect(addr).expect("connect function failed");
    // socket.send_to("connect".as_bytes(),&addr).unwrap();
    socket.send("connect".as_bytes()).expect("message sending failed"); // when connected, can just send not send_to
    read_incoming_messages(socket);
}
 
fn read_incoming_messages(socket :&UdpSocket) {
    let mut buf = [0; 2048];
	let (amt, _src) = socket.recv_from(&mut buf).expect("incoming message failed");
    let filled_buf = &mut buf[..amt];
    let incoming_message = String::from_utf8_lossy(filled_buf).into_owned();

    println!("SERVER --> {:?}", incoming_message);
    if incoming_message.contains(""){

    }
}

