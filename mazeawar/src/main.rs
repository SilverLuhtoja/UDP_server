use crate::client_server::*;
use crate::map::Map;
use crate::maze::{Grid, LOW};
use crate::player::*;
use local_ip_address::local_ip;
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use serde_json::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::process::exit;

mod client_server;
mod map;
mod maze;
mod player;

#[derive(Serialize, Deserialize)]
struct Data {
    map: Map,
    location: Point,
}

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

#[macroquad::main(window_conf)]
async fn main() -> std::io::Result<()> {
    let server_addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 4242);

    let spawn_point = Point::new(20.0, 20.0);
    let mut player = Player::new(spawn_point);

    let client = Client::new(ADDR);
    client.socket.connect(server_addr).expect("Connecting With Server Failed!");
    println!("Creating server : {:?}.Listening....", client);

    let mut message = Message {
        message_type: "connect".to_string(),
        data: json!(""),
    };
    client.socket.send(serde_json::to_string(&message)?.as_bytes())?;

    let mut data: Data = serde_json::from_str(&read_incoming_messages(&client.socket))?;
    let map = data.map;
    player.set_postion(data.location);
    loop {
        map.draw();
        player.draw(); // allPlayers update (including player itself)

        if is_key_pressed(KeyCode::A) {
            let point = Point::new(player.location.x - 20.0, player.location.y);
            message = Message {message_type: "movement".to_string(),data: json!(point),};
            client.socket.send(serde_json::to_string(&message)?.as_bytes())?;

            data = serde_json::from_str(&read_incoming_messages(&client.socket))?;
            player.set_postion(data.location);
        }
        if is_key_pressed(KeyCode::D) {
            let point = Point::new(player.location.x + 20.0, player.location.y);
            message = Message {message_type: "movement".to_string(),data: json!(point),};
            client.socket.send(serde_json::to_string(&message)?.as_bytes())?;

            data = serde_json::from_str(&read_incoming_messages(&client.socket))?;
            player.set_postion(data.location);
        }
        if is_key_pressed(KeyCode::W) {
            let point = Point::new(player.location.x, player.location.y - 20.0);
            message = Message {message_type: "movement".to_string(),data: json!(point),};
            client.socket.send(serde_json::to_string(&message)?.as_bytes())?;

            data = serde_json::from_str(&read_incoming_messages(&client.socket))?;
            player.set_postion(data.location);
        }
        if is_key_pressed(KeyCode::S) {
            let point = Point::new(player.location.x, player.location.y + 20.0);
            message = Message {message_type: "movement".to_string(),data: json!(point),};
            client.socket.send(serde_json::to_string(&message)?.as_bytes())?;

            data = serde_json::from_str(&read_incoming_messages(&client.socket))?;
            player.set_postion(data.location);
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
