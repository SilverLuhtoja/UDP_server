mod map;
mod maze;

use serde::{Deserialize, Serialize};
use serde_json::{json};
use serde_json::Value as JsonValue;
use std::net::{UdpSocket, SocketAddr};
use crate::{maze::{Grid, LOW}};
use std::collections::HashMap;

const ADDR: &str = "127.0.0.1";
const PORT: u16 = 4242;

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug, PartialEq,Deserialize, Serialize)]
struct Data{
    message_type: String,
    data: JsonValue,
}

#[derive(Clone, Debug, PartialEq,Deserialize, Serialize)]
struct BroadcastMessage{
    map: JsonValue,
    location: Point,
}

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind(format!("{}:{}", ADDR, PORT))?;
    let mut buf = [0; 24000];
    let mut grid = Grid::new(10, 10, LOW);
    grid.generate_maze();
    let map = grid.convert_to_map();
    let mut connected_players:HashMap<SocketAddr, BroadcastMessage> = HashMap::new();
    println!("Creating server : {:?}.Listening....", socket);

    loop {
        println!();
        // (bite_slice, address where it came from)
        let (amt, src) = socket.recv_from(&mut buf).expect("incoming message failed");
        let incoming_message = String::from_utf8_lossy(&mut buf[..amt]);
        println!("client <{}>: {:?}", src, incoming_message);

        let data: Data = serde_json::from_str(&incoming_message)?;
        
        if data.message_type == "movement" {
            let current_player = connected_players.get_mut(&src).expect("ADD PLAYER < NOT IN HASH >");
            let point:Point = serde_json::from_value(data.data)?;
            current_player.location = point;
           
        }
        if data.message_type == "connect" {
            // let message = json!({
            // "map" : map,
            // "location" : player
            // });
            let multiplier = (connected_players.len() + 1) as f32;
            let location = Point { x: 100.0 * multiplier, y: 100.0 * multiplier};
            let message = BroadcastMessage{
                map: json!(map),
                location
            };


            connected_players.insert(src, message);
            // socket
            //     .connect(&src)
            //     .expect("SERVER: connect function failed");
            // socket.send(json!(&message).to_string().as_bytes())?;

           
        }


        for (addr,v) in &connected_players{
            socket.send_to(json!(&v).to_string().as_bytes(), addr)?;
        }
    }
}
