use std::io::stdin;
use std::env;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod tokens;
mod scanner;

fn input() -> String {
    let mut buf = String::new();

    stdin().read_line(&mut buf).unwrap();

    buf
}



fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        println!("Usage: jlox [script]");
        todo!()
    } else if args.len() == 1 {
        todo!()
    } else {
        todo!()
    }
}
