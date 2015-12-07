#![feature(test)]
#![feature(iter_arith)]
#![feature(convert)]
#[macro_use]

extern crate nom;

mod day2;

use std::io::{stdin, Read};



fn run_day2() {
    let mut buffer : Vec<u8> = vec![];
    stdin().read_to_end(&mut buffer).unwrap();
    let req = day2::paper_required(&buffer[..]);
    let rib = day2::ribbon_required(&buffer[..]);
    println!("[Day 2] Part 1: {}", req);
    println!("[Day 2] Part 2: {}", rib);
}

pub fn main() {
    run_day2();
}
