extern crate go;

use std::fs::File;
use std::io::Read;
use std::env;

use go::sgf;

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut f = File::open(&args[1]).expect("file open");
    let mut txt = Vec::new();
    f.read_to_end(&mut txt).expect("read");

    let sgfcoll = sgf::parser(&txt[..]).expect("parse");
    println!("sgf {:?}", sgfcoll);
}
