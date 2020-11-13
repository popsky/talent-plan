use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;

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
    {
        let file = File::create("output.json").unwrap();
        let mut writer = BufWriter::new(file);

        let m = Move {
            step: 1,
            direction: Direction::Up,
        };
        let contents = serde_json::to_string(&m).unwrap();
        writer.write(contents.as_bytes()).unwrap();
    }

    {
        let file = File::open("output.json").unwrap();
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();

        reader.read_to_string(&mut buffer).unwrap();
        let m: Move = serde_json::from_str(&buffer).unwrap();
        println!("{:?}", m);
    }
}
