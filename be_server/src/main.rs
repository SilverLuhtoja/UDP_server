mod map;
mod maze;

use serde::{Deserialize, Serialize};
use serde_json::{json};
use serde_json::Value as JsonValue;
use std::net::UdpSocket;
use crate::{maze::{Grid, LOW}, map::Map};

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
    println!("Creating server : {:?}.Listening....", socket);

    let mut location = Point { x: 200.0, y: 200.0 };
    loop {
        println!();
        // (bite_slice, address where it came from)
        let (amt, src) = socket.recv_from(&mut buf).expect("incoming message failed");
        let incoming_message = String::from_utf8_lossy(&mut buf[..amt]);
        println!("client <{}>: {:?}", src, incoming_message);

        let data: Data = serde_json::from_str(&incoming_message)?;
        if data.message_type == "movement" {
            let point:Point = serde_json::from_value(data.data)?;
            location = point;
            socket.send(json!(&location).to_string().as_bytes())?;
        }
        if data.message_type == "connect" {
            // let message = json!({
            // "map" : map,
            // "location" : player
            // });
            let message = BroadcastMessage{
                map: json!(map),
                location
            };
            socket
                .connect(&src)
                .expect("SERVER: connect function failed");
            socket.send(json!(&message).to_string().as_bytes())?;
        }
    }
}
