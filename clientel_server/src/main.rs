use std::io::{self, BufRead};
use std::net::UdpSocket;
use std::str;
use macroquad::prelude::*;
use clientel_server::*;
use std::collections::HashMap;


const ADDR:&str = "127.0.0.1";
const SERVER_PORT:u16 = 4242;
const CLIENT_PORT:u16 = 34254;

// static map: Vec<u8>;

fn window_conf() -> Conf {
    Conf {
        window_title: "MAZE WARS".to_owned(),
        window_width: 1200,
        window_height: 800,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind(format!("{}:{}",ADDR, CLIENT_PORT))?;
    println!("Establishing UDP socket address at : {:?}", socket);
    try_to_connect(&socket); // "fake handshake"
    let map: Vec<Vec<i32>> = vec![
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1],
        vec![1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1],
        vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];

    let mut players = HashMap::new();
    players.insert(String::from("Anna"), String::from("30"));
    players.insert(String::from("Silver"), String::from("20"));
    players.insert(String::from("Valeria"), String::from("10"));
    players.insert(String::from("Emil"), String::from("05"));


    loop{
        clear_background(Color::new(0.0, 0.0, 0.0, 0.8));

        let game_board = GameBoard::new(map.clone());
        let visual_board = MazeVisual::new(game_board.clone());
        let score_board = ScoreBoard::new(visual_board.clone(), players.clone());
        game_board.draw();
        visual_board.draw();
        score_board.draw();
        

        if is_key_pressed(KeyCode::Left) {
            println!("\nwrite to server:");
            socket.send("Left".as_bytes()).expect("Error on send");
        }

        if is_key_pressed(KeyCode::Right) {
            println!("\nwrite to server:");
            socket.send("Right".as_bytes()).expect("Error on send");
        }

        if is_key_pressed(KeyCode::Up) {
            println!("\nwrite to server:");
            socket.send("Up".as_bytes()).expect("Error on send");
        }

        if is_key_pressed(KeyCode::Down) {
            println!("\nwrite to server:");
            socket.send("Down".as_bytes()).expect("Error on send");
        }

        if is_key_pressed(KeyCode::Space) {
            println!("\nwrite to server:");
            socket.send("Shoot".as_bytes()).expect("Error on send");
        }

        // let stdin = io::stdin();
        // for line in stdin.lock().lines() {
        //     let line = line.unwrap();
        //     if &line == "BYE" || &line == "bye" {
        //         socket.send("stop".as_bytes()).expect("Error on send");
        //         break;
        //     }
        //     socket.send(line.as_bytes()).expect("Error on send");
        //     read_incoming_messages(&socket);
        // }

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



// fn draw_the_score(){
//     draw_text("SCORE:", 620.0, 700.0, 20.0, WHITE);
//     draw_line(700.0, 620.0, 1180.0, 620.0, 1.0, RED); //top
//     draw_line(700.0, 780.0, 1180.0, 780.0, 1.0, RED); //bottom
//     draw_line(700.0, 620.0, 700.0, 780.0, 1.0, RED); //left
//     draw_line(1180.0, 620.0, 1180.0, 780.0, 1.0, RED); //right
// }