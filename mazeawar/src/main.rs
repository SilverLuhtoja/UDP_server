use crate::client_server::*;
use crate::map::Map;
use crate::maze::{Grid, LOW};
use crate::player::*;
use local_ip_address::local_ip;
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use serde_json::*;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::process::exit;

mod client_server;
mod map;
mod maze;
mod player;

#[derive(Serialize, Deserialize)]
struct Data {
    map: Map,
    players: HashMap<SocketAddr, Player>,
}
// #[derive(Serialize, Deserialize)]
// struct Data {
//     map: Map,
//     location: Point,
// }

fn window_conf() -> Conf {
    Conf {
        window_title: "MAZE".to_owned(),
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct Message {
    message_type: String,
    data: JsonValue,
}

// const ADDR: &str = "127.0.0.1";
const ADDR: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const TIMER: u8 = 60;

#[macroquad::main(window_conf)]
async fn main() -> std::io::Result<()> {
    let server_addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 4242);

    // let spawn_point = Point::new(20.0, 20.0);
    // let mut player = Player::new(spawn_point);
    // let spawn_point2 = Point::new(50.0, 50.0);
    // let mut player2 = Player::new(spawn_point2);

    let mut players: HashMap<SocketAddr, Player> = HashMap::new();
 
    let client = Client::new(ADDR);
    let my_addr = "127.0.0.1:34254";
    client.socket.connect(server_addr).expect("Connecting With Server Failed!");
    println!("Creating server : {:?}.Listening....", client);

    let mut message = Message {
        message_type: "connect".to_string(),
        data: json!(""),
    };
    client.socket.send(serde_json::to_string(&message)?.as_bytes())?;

    let data: Data = serde_json::from_str(&read_incoming_messages(&client.socket))?;
    let map = data.map;
    players = data.players;
    let mut  my_point = Point::new(0.0,0.0);
    // player.set_postion(data.location);
    let mut fps = TIMER;
    loop {
        map.draw();
        // allPlayers update (including player itself)
        for (k,player) in &players{ 
            if k.to_string() == my_addr{
                my_point = player.location;
            }
            player.draw();
        }
        // player.draw();
        println!("{}", fps);
        fps -= 1;
        if fps == 0 {
            println!("Sending Update now");
            message = Message {message_type: "update".to_string(),data: json!(""),};
            client.socket.send(serde_json::to_string(&message)?.as_bytes())?;

            let data: Data = serde_json::from_str(&read_incoming_messages(&client.socket))?;
            players = data.players;
            fps=TIMER;
        }



        if is_key_pressed(KeyCode::A) {
            let point = Point::new(my_point.x - 20.0, my_point.y);
            message = Message {message_type: "movement".to_string(),data: json!(point),};
            client.socket.send(serde_json::to_string(&message)?.as_bytes())?;

            // data = serde_json::from_str(&read_incoming_messages(&client.socket))?;
            // player.set_postion(data.location);
        }
        if is_key_pressed(KeyCode::D) {
            let point = Point::new(my_point.x + 20.0, my_point.y);
            message = Message {message_type: "movement".to_string(),data: json!(point),};
            client.socket.send(serde_json::to_string(&message)?.as_bytes())?;

            // data = serde_json::from_str(&read_incoming_messages(&client.socket))?;
            // player.set_postion(data.location);
        }
        if is_key_pressed(KeyCode::W) {
            let point = Point::new(my_point.x, my_point.y - 20.0);
            message = Message {message_type: "movement".to_string(),data: json!(point),};
            client.socket.send(serde_json::to_string(&message)?.as_bytes())?;

            // data = serde_json::from_str(&read_incoming_messages(&client.socket))?;
            // player.set_postion(data.location);
        }
        if is_key_pressed(KeyCode::S) {
            let point = Point::new(my_point.x, my_point.y + 20.0);
            message = Message {message_type: "movement".to_string(),data: json!(point),};
            client.socket.send(serde_json::to_string(&message)?.as_bytes())?;

            // data = serde_json::from_str(&read_incoming_messages(&client.socket))?;
            // player.set_postion(data.location);
        }

        if is_key_pressed(KeyCode::Escape) {
            exit(1)
        }
        next_frame().await
    }
}

pub fn read_incoming_messages(socket: &UdpSocket) -> String {
    let mut buf = [0; 24000];
    let (amt, _src) = socket.recv_from(&mut buf).expect("incoming message failed");
    let filled_buf = &mut buf[..amt];
    let incoming_message = String::from_utf8_lossy(filled_buf).into_owned();
    // println!("SERVER --> {:?}", incoming_message);
    incoming_message
}
