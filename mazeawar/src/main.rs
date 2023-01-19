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
    let client = Client::new(ADDR);
    let spawn_point = Point::new(100.0, 100.0);
    let mut player = Player::new(spawn_point);
    let mut grid = Grid::new(10, 10, LOW);
    grid.generate_maze();
    let map = grid.convert_to_map();

    println!("Creating server : {:?}.Listening....",client);
    client.socket.send_to("connect".as_bytes(), server_addr)?;

    let point: Point = serde_json::from_str(&read_incoming_messages(&client.socket))?;
    player.set_postion(point);
    loop {
        map.draw();
        player.draw(); // allPlayers update (including player itself)

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
