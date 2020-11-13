use bson::*;
use bson::{oid, Array, Bson, Document};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Cursor;

#[derive(Serialize, Deserialize, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Serialize, Deserialize, Debug)]
struct Move {
    speed: i32,
    direction: Direction,
}

fn to_vec() -> std::io::Result<()> {
    let mut writer = Vec::new();
    for i in 1..1001 {
        let m = Move {
            speed: i,
            direction: Direction::Up,
        };
        let bson = bson::to_bson(&m).unwrap();
        let doc = bson.as_document().unwrap();
        doc.to_writer(&mut writer).unwrap();
    }
    let mut reader = &mut Cursor::new(&writer[..]);
    for i in 1..1001 {
        let doc = Document::from_reader(&mut reader).unwrap();
        let mut m: Move = bson::from_document(doc).unwrap();
        println!("{:?}", m);
    }

    Ok(())
}
fn main() -> std::io::Result<()> {
    // to_vec();
    let file = File::create("output.bson").unwrap();
    let mut writer = BufWriter::new(file);
    let f = File::open("output.bson")?;
    let mut reader = BufReader::new(f);

    for i in 1..1001 {
        let m = Move {
            speed: i,
            direction: Direction::Up,
        };
        let bson = bson::to_bson(&m).unwrap();
        let doc = bson.as_document().unwrap();
        doc.to_writer(&mut writer).unwrap();
    }
    writer.flush().unwrap();

    // let f = File::open("output.bson")?;
    // let mut reader = BufReader::new(f);
    for i in 1..1001 {
        let doc = Document::from_reader(&mut reader).unwrap();
        let mut m: Move = bson::from_document(doc).unwrap();
        println!("{:?}", m);
    }

    Ok(())
}
