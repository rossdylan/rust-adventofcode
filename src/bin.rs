#![feature(test)]

#![feature(iter_arith)]
#![feature(convert)]
#[macro_use]

extern crate nom;
extern crate clap;

mod day2;

use std::io::{stdin, Read};
use clap::{App, SubCommand};


fn run_day2() {
    let mut buffer : Vec<u8> = vec![];
    stdin().read_to_end(&mut buffer).unwrap();
    let req = day2::paper_required(&buffer[..]);
    let rib = day2::ribbon_required(&buffer[..]);
    println!("[Day 2] Part 1: {}", req);
    println!("[Day 2] Part 2: {}", rib);
}

fn run_day3() {
    let mut buffer : Vec<u8> = vec![];
    stdin().read_to_end(&mut buffer).unwrap();
}

pub fn main() {
    let matches = App::new("adventofcode")
        .version("0.1.0")
        .author("Ross Delinger <rossdylan@fastmail.com>")
        .about("Program for running my adventofcode submissions")
        .subcommand(SubCommand::with_name("day2"))
        .subcommand(SubCommand::with_name("day3")).get_matches();

    if let Some(_) = matches.subcommand_matches("day2") {
        run_day2();
    }
    if let Some(_) = matches.subcommand_matches("day3") {
        run_day3();
    }
}
