use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};
use std::process::exit;

use crate::maze::{Grid, LOW};
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value as JsonValue;
use serde_json::*;
use tokio::join;
use tokio::net::UdpSocket;

use crate::client_server::*;
use crate::map::Map;
use crate::player::*;

mod client_server;
mod map;
mod maze;
mod player;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct Data {
    map: Map,
    players: HashMap<SocketAddr, Player>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct Message {
    message_type: String,
    data: JsonValue,
}

enum Gamestate {
    CONNECT,
    START
}

fn window_conf() -> Conf {
    Conf {
        window_title: "MAZE".to_owned(),
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}

pub async fn create_client() -> std::io::Result<UdpSocket> {
    let client = UdpSocket::bind("0.0.0.0:0").await?;
    Ok(client)
}

#[macroquad::main(window_conf)]
async fn main() {
    let server_addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 5, 0, 2)), 4242);
    let mut buf = [0; 6000];
    // let client = create_client().await.unwrap();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut connected = false;
    let mut message = Message {
        message_type: "connect".to_string(),
        data: json!(""),
    };
    
    loop {
        let data = rt.block_on(async {
            if connected{
                message.message_type = "update".to_string();
            }
            // This needs to change
            let client = UdpSocket::bind("10.5.0.2:34254").await.unwrap();
            client.connect(server_addr).await.unwrap();
            client.send(to_string(&message).unwrap().as_bytes()).await.unwrap();
            connected = true;

            let (recv_len, _) = client.recv_from(&mut buf).await.unwrap();
            let incoming_message = String::from_utf8_lossy(&buf[..recv_len]);
            let data: Data = serde_json::from_str(&incoming_message).unwrap();

            println!("MESSAGE ");

            return data;
        });
        

        data.map.draw();
        for (_, player) in &data.players {
            player.draw();
        }

        if is_key_pressed(KeyCode::Escape) {
            exit(1)
        }
        next_frame().await;
    }
}

// #[macroquad::main(window_conf)]
// async fn main() -> std::io::Result<()> {
//     let server_addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 5, 0, 2)), 4242);
//     let mut rt = tokio::runtime::Runtime::new().unwrap();
//     rt.block_on(async {
//         let client = UdpSocket::bind(format!(
//             "{}",
//             SocketAddrV4::new(std::net::Ipv4Addr::new(0, 0, 0, 0), 0)
//         ))
//         .await?;

//         client.connect(server_addr).await?;
//         println!("Creating client-server : {:?}.Listening....", client);

//         let my_addr = client.local_addr().unwrap().to_string();

//         let mut message = Message {
//             message_type: "connect".to_string(),
//             data: json!(""),
//         };
//         client.send(to_string(&message)?.as_bytes()).await?;

//         let value = &read_incoming_messages(&client).await?;
//         let data: Data = from_str(&value)?;
//         let map = data.map;
//         let mut players: HashMap<SocketAddr, Player> = data.players;

//         loop {
//             map.draw();
//             for (_, player) in &players {
//                 player.draw();
//             }

//             message = Message {
//                 message_type: "update".to_string(),
//                 data: json!(""),
//             };
//             client.send(to_string(&message)?.as_bytes()).await?;
//             let data: Data = from_str(&read_incoming_messages(&client).await?)?;
//             players = data.players;

//             if is_key_pressed(KeyCode::Escape) {
//                 exit(1)
//             }
//             next_frame().await
//         }
//     })
// }

// pub async fn read_incoming_messages(socket: &UdpSocket) -> std::io::Result<String> {
//     let mut buf = [0; 24000];
//     let (amt, _src) = socket.recv_from(&mut buf).await?;
//     let filled_buf = &mut buf[..amt];
//     let incoming_message = String::from_utf8_lossy(filled_buf).into_owned();
//     // println!("SERVER --> {:?}", incoming_message);
//     Ok(incoming_message)
// }

// if is_key_pressed(KeyCode::A) {
//     let point = Point::new(my_point.x - 20.0, my_point.y);
//     message = Message {
//         message_type: "movement".to_string(),
//         data: json!(point),
//     };
//     client.send(to_string(&message)?.as_bytes()).await?;
// }
// if is_key_pressed(KeyCode::D) {
//     let point = Point::new(my_point.x + 20.0, my_point.y);
//     message = Message {
//         message_type: "movement".to_string(),
//         data: json!(point),
//     };
//     client.send(to_string(&message)?.as_bytes()).await?;
// }
// if is_key_pressed(KeyCode::W) {
//     let point = Point::new(my_point.x, my_point.y - 20.0);
//     message = Message {
//         message_type: "movement".to_string(),
//         data: json!(point),
//     };
//     client.send(to_string(&message)?.as_bytes()).await?;
// }
// if is_key_pressed(KeyCode::S) {
//     let point = Point::new(my_point.x, my_point.y + 20.0);
//     message = Message {
//         message_type: "movement".to_string(),
//         data: json!(point),
//     };
//     client.send(to_string(&message)?.as_bytes()).await?;
// }
