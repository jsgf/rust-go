extern crate go;

use std::fs::File;
use std::io::Read;
use std::env;

use go::sgf;
use go::sgf::property::{Value};
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

    let mut movenum = 1;
    while node.movenode() {
        for &(p, c) in &[("B", Stone::Black), ("W", Stone::White)] {
            let p = node.prop(p).and_then(|p| p.value().ok());
            let m: Option<Location> = p.and_then(|v| v.gomove().map(Into::into));
            if let Some(loc) = m {
                let loc = Location::new(loc.col(), sz - 1 - loc.row());
                println!("Move {}: {:?} {}", movenum, c, loc);
                movenum += 1;
                if !board.play(loc, c) {
                    println!("bad play: {} {:?}", loc, c)
                } else {
                    println!("{}", board);

                    for g in board.groups::<Vec<_>>(c) {
                        print!("{:?} group: {} liberties: [", c, g);
                        for l in board.liberties::<Vec<_>>(&g) {
                            print!(" {}", l)
                        }
                        println!(" ]");
                    }
                }
            }
        }

        if node.len() == 0 { break }
        node = &node[0];
    }

    println!("Board:\n{}", board);
}
