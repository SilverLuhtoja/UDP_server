pub use std::fs::read;
pub use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
pub use serde_json::*;
pub use std::net::SocketAddr;
pub use std::process::exit;
pub use std::sync::Arc;
pub use std::sync::mpsc::channel;
pub use std::thread;
pub use local_ip_address::local_ip;

pub use common::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
pub use map::map::Map;
pub use map::game_window::GameWindow;
pub use player::{player::*, movement::reverse_difference};
pub use utils::utils::{client_input::read_input, convert::to_ip, client_input::InputType};
pub use utils::point::Point;
pub use crate::client_server::*;

pub mod client_server;
pub mod map;
pub mod common;
pub mod player;
pub mod utils;

#[derive(Clone, Debug, PartialEq,Deserialize, Serialize, Default)]
pub enum GameState{
    #[default]
    Game,
    Killed,
    NewLevel
}
