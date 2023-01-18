use crate::maze::{Grid, LOW};
use crate::map::Map;
use crate::player::*;
use crate::client_server::*;
use local_ip_address::local_ip;
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::*;
use std::net::{UdpSocket, IpAddr, Ipv4Addr, SocketAddr};
use std::process::exit;

mod client_server;
mod map;
mod maze;
mod player;

#[derive(Serialize,Deserialize)]
struct Data {
    map: Map,
    player : Point
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Message {
    pub messsage_type: String,
    pub y: f32,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "MAZE".to_owned(),
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}

// const ADDR: &str = "127.0.0.1";
const ADDR: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

#[macroquad::main(window_conf)]
async fn main() -> std::io::Result<()> {
    let server_addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 4242);

    let spawn_point = Point::new(100.0, 100.0);
    let mut player = Player::new(spawn_point);

    let mut message = json!({
        "message_type" : "movement",
        "point" : ""
    });

    let client = Client::new(ADDR);
    println!("Creating server : {:?}.Listening....",client);
    client.socket.send_to("connect".as_bytes(), server_addr)?;

    let server_value = read_incoming_messages(&client.socket);
    // let point: Point = serde_json::from_str(&server_value)?;
    let data: Data = serde_json::from_str(&server_value)?;
    let map = data.map;
    // player.set_postion(point);
    loop {
        map.draw();

        // player.draw(); // allPlayers update (including player itself)


        if is_key_pressed(KeyCode::A) {
            let point = Point::new(player.location.x - 20.0, player.location.y);
            message["point"] = json!(point);
            client.socket.send_to(serde_json::to_string(&message)?.as_bytes(), server_addr)?;

            let returned_server_point: Point =
                serde_json::from_str(&read_incoming_messages(&client.socket))?;
            player.set_postion(returned_server_point);
        }
        if is_key_pressed(KeyCode::D) {
            let point = Point::new(player.location.x + 20.0, player.location.y);
            message["point"] = json!(point);
            client.socket.send_to(serde_json::to_string(&message)?.as_bytes(), server_addr)?;

            let returned_server_point: Point =
                serde_json::from_str(&read_incoming_messages(&client.socket))?;
            player.set_postion(returned_server_point);
        }
        if is_key_pressed(KeyCode::W) {
            let point = Point::new(player.location.x, player.location.y - 20.0);
            message["point"] = json!(point);
            client.socket.send_to(serde_json::to_string(&message)?.as_bytes(), server_addr)?;

            let returned_server_point: Point =
                serde_json::from_str(&read_incoming_messages(&client.socket))?;
            player.set_postion(returned_server_point);
        }
        if is_key_pressed(KeyCode::S) {
            let point = Point::new(player.location.x, player.location.y + 20.0);
            message["point"] = json!(point);
            client.socket.send_to(serde_json::to_string(&message)?.as_bytes(), server_addr)?;

            let returned_server_point: Point =
                serde_json::from_str(&read_incoming_messages(&client.socket))?;
            player.set_postion(returned_server_point);
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
    println!("SERVER --> {:?}", incoming_message);
    incoming_message
}
