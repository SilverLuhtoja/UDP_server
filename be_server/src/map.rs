use serde::{Deserialize, Serialize};

pub const WALL: i32 = 1;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Map(pub Vec<Vec<i32>>);

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Self(vec![vec![0; width]; height])
    }
}
