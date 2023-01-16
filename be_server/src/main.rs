use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::net::UdpSocket;

const ADDR: &str = "127.0.0.1";
const PORT: u16 = 4242;

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Data<T> {
    pub messsage_type: String,
    pub data: Option<T>,
}

fn main() -> std::io::Result<()> {
    // for UDP4
    // let socket = UdpSocket::bind("[::]:2000")?;  // for UDP4/6
    let socket = UdpSocket::bind(format!("{}:{}", ADDR, PORT))?;
    let mut buf = [0; 2048];
    println!("Creating server : {:?}.Listening....", socket);
    println!("Waiting messages:");

    let mut player = Point { x: 200.0, y: 200.0 };
    let mut flag = false;
    loop {
        println!();
        // (bite_slice, address where it came from)
        let (amt, src) = socket.recv_from(&mut buf).expect("incoming message failed");
        let incoming_message = String::from_utf8_lossy(&mut buf[..amt]);
        println!("client <{}>: {:?}", src, incoming_message);

        // currently it throws error without flag, cuz it cant read data.field
        if flag {
            let data: Value = serde_json::from_str(&incoming_message)?;
            if data["message_type"] == "movement" {
                player.x = data["point"]["x"].as_f64().unwrap() as f32;
                player.y = data["point"]["y"].as_f64().unwrap() as f32;
            }
            socket.send(json!(&player).to_string().as_bytes())?;
        }

        if incoming_message == "connect" {
            // let message  = format!("Successful connection with {}:{}",ADDR,PORT);
            // socket.send(message.as_bytes());
            socket
                .connect(&src)
                .expect("SERVER: connect function failed");
            socket.send(json!(&player).to_string().as_bytes())?;
            flag = true
            // socket.send_to(message.as_bytes(), &src)?;
        }
    }
}
