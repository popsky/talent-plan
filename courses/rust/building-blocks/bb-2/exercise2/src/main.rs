use ron;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::str;

#[derive(Serialize, Deserialize, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Serialize, Deserialize, Debug)]
struct Move {
    step: u32,
    direction: Direction,
}
fn main() {
    let m = Move {
        step: 1,
        direction: Direction::Up,
    };
    let contents_ron = ron::to_string(&m).unwrap().into_bytes();
    let contents_json = serde_json::to_string(&m).unwrap().into_bytes();

    println!("ron: {}", str::from_utf8(&contents_ron).unwrap());
    println!("json: {}", str::from_utf8(&contents_json).unwrap());
}
