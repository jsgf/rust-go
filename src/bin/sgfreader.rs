extern crate go;

use std::fs::File;
use std::io::Read;
use std::env;

use go::sgf;
use go::sgf::property::{Value, Number};
use go::board::Board;
use go::stone::Stone;
use go::location::Location;

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut f = File::open(&args[1]).expect("file open");
    let mut txt = Vec::new();
    f.read_to_end(&mut txt).expect("read");

    let sgfcoll = sgf::parser(&txt[..]).expect("parse");
    let mut node = &sgfcoll[0];
    let mut sz = None;

    while !node.movenode() {
        println!("root {:?} setup {:?} move {:?}", node.rootnode(), node.setupnode(), node.movenode());

        for v in node.properties() {
            println!("{}: {:?}", v.id(), v.values())
        }

        sz = match &node["SZ"].values().unwrap()[0] {
            &Value::Number(ref n) => Some(n.into()),
            x => { println!("bad sz {:?}", x); None },
        };

        node = &node[0];
    }

    if sz.is_none() {
        println!("no size");
        return;
    }

    let sz = sz.unwrap();

    let mut board = Board::new_with_size(sz);

    while node.movenode() {
        if let Some(p) = node.prop("B").map(|p| p.value().unwrap()).map(Location::from) {
            let _ = board.add(p, Stone::Black);
        }
        if let Some(p) = node.prop("W").map(|p| p.value().unwrap()).map(Location::from) {
            let _ = board.add(p, Stone::White);
        }
    }
}
