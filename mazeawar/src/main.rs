// #![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};
use std::process::exit;
use std::sync::{Mutex, Arc};
use std::sync::mpsc::{channel, Sender};

use crate::maze::{Grid, LOW};
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value as JsonValue;
use serde_json::*;
use tokio::join;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use std::thread;
use std::time::Duration;
use tokio::task;

use crate::client_server::*;
use crate::map::Map;
use crate::player::*;

mod client_server;
mod map;
mod maze;
mod player;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Data {
    map: Map,
    players: HashMap<SocketAddr, Player>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Message {
     message_type: String,
     data: JsonValue,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "MAZE".to_owned(),
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let server_addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 5, 0, 2)), 4242);
    let mut buf = [0; 6000];
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let message = Message {
        message_type: "connect".to_string(),
        data: json!(""),
    };
    // let (tx, mut rx) = mpsc::channel::<(Vec<u8>, SocketAddr)>(1_000);
    // runtime.block_on(async move {
    //     let sock =  UdpSocket::bind("10.5.0.2:34255").await.unwrap();
    //     let r = Arc::new(sock);
    //     let s = r.clone();

    //     tokio::spawn(async move{
    //         while let Some((bytes,addr)) = rx.recv().await{
    //             let len = s.send_to(&bytes, &server_addr).await.unwrap();
    //             println!("{:?} bytes sent", len);
    //         }
    //     });

    //     let mut buf = [0; 1024];
    //     loop {
    //         let (len, addr) = r.recv_from(&mut buf).await.unwrap();
    //         println!("{:?} bytes received from {:?}", len, addr);
    //         tx.send((buf[..len].to_vec(), addr)).await.unwrap();
    //     }
    // });


    //transmitter, receiver
    // let (tx, rx) = mpsc::channel(10);
    // let data =rx.recv().await.unwrap();

    
    

    // println!("Got: {:?}", data);
    
    let mut my_point = Point::new(0.0, 0.0);
    let mut  message = Message {
           message_type: "connect".to_string(),
           data: json!(""),
       };
    ok(&message);
    message.message_type = "update".to_string();
    loop {
        let data:Data = ok(&message);
        
        data.map.draw();
        for (src, player) in &data.players {
            if src.to_string() == "10.5.0.2:34255"{
                my_point = player.location
            }
            player.draw();
        }
        
        if is_key_pressed(KeyCode::Escape) {
            exit(1)
        }

        if is_key_pressed(KeyCode::A) {
            let point = Point::new(my_point.x - 20.0, my_point.y);
            let message = Message { message_type: "movement".to_string(), data: json!(point) };
            ok(&message);
        }

        if is_key_pressed(KeyCode::D) {
            let point = Point::new(my_point.x + 20.0, my_point.y);
            let message = Message { message_type: "movement".to_string(), data: json!(point) };
            ok(&message);
        }

        next_frame().await;
    }
}

pub fn ok(message: &Message) -> Data{
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let mut buf = [0; 6000];
    return runtime.block_on(async {
            let client = UdpSocket::bind("10.5.0.2:34255").await.unwrap();
            client.connect("10.5.0.2:4242").await.unwrap();
            client.send(to_string(&message).unwrap().as_bytes()).await.unwrap();
        
            let (recv_len, _) = client.recv_from(&mut buf).await.unwrap();
            let incoming_message = String::from_utf8_lossy(&buf[..recv_len]);
            let data: Data = serde_json::from_str(&incoming_message).unwrap();
            
            return data
    });
}

// runtime.block_on(async {
//     tokio::spawn(async move {
//         let client = UdpSocket::bind("10.5.0.2:34255").await.unwrap();
//         client.connect(server_addr).await.unwrap();

//         loop{
//             client.send(to_string(&message).unwrap().as_bytes()).await.unwrap();
        
//             let (recv_len, _) = client.recv_from(&mut buf).await.unwrap();
//             let incoming_message = String::from_utf8_lossy(&buf[..recv_len]);
//             let data: Data = serde_json::from_str(&incoming_message).unwrap();
            
//             println!("MESSAGE ");
//             tx.send(data).await.unwrap();
//         }
//     });
// });

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
