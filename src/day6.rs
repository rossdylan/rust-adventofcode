use nom::IResult;
use nom::digit;
use nom::IResult::*;
use std::collections::HashMap;
use std::str;
use std::str::FromStr;


#[derive(Debug,Clone,Copy)]
pub enum Operation {
    On,
    Off,
    Toggle
}

pub type Instruction = (Operation, (i64, i64), (i64, i64));
pub type LightGrid = HashMap<(i64,i64), usize>;

pub fn op_from_str(string : &[u8]) -> Result<Operation, &str> {
    match str::from_utf8(string).unwrap() {
        "turn on " => Ok(Operation::On),
        "turn off " => Ok(Operation::Off),
        "toggle " => Ok(Operation::Toggle),
        _ => Err("Wat")
    }
}



named!(turn_on, tag!("turn on "));
named!(toggle, tag!("toggle "));
named!(turn_off, tag!("turn off "));
named!(op_parser<&[u8], Operation>,
   map_res!(
       alt!(
           turn_on |
           turn_off |
           toggle
        ),
       op_from_str
    )
);

named!(number<i64>,
    map_res!(
        map_res!(
            digit,
            str::from_utf8
        ),
        FromStr::from_str
    )
);

named!(square<&[u8], (i64, i64)>,
    chain!(
        x: number ~
        tag!(",") ~
        y: number ,
        || (x, y)
    )
);

named!(parse_instr<&[u8], Instruction>,
   chain!(
       op: op_parser ~
       start: square ~
       tag!(" through ") ~
       stop: square ~
       tag!("\n") ,
       || (op, start, stop)
    )
);

named!(all_instr<&[u8], Vec<Instruction> >, many0!(parse_instr));

pub fn modify_light(grid : &mut LightGrid, op : Operation, start : (i64,i64), stop : (i64,i64)) {
    for x in start.0..(stop.0+1) {
        for y in start.1..(stop.1+1) {
            match op {
                Operation::On => {
                    let new = match grid.get_mut(&(x,y)) {
                        Some(val) => *val+1,
                        None => 1
                    };
                    grid.insert((x,y), new)
                }
                Operation::Off => {
                    let new = match grid.get_mut(&(x,y)) {
                        Some(val) => {
                            if *val == 0 {
                                0
                            } else {
                                *val-1
                            }
                        },
                        None => 0
                    };
                    grid.insert((x,y), new)
                }
                Operation::Toggle => {
                    let new = match grid.get_mut(&(x,y)) {
                        Some(val) => *val+2,
                        None => 2
                    };
                    grid.insert((x,y), new)
                }
            };
        }
    }
}


pub fn parse_instructions(input: &[u8]) -> Vec<Instruction> {
    match all_instr(input) {
        IResult::Done(_, res) => res,
        IResult::Error(_) => vec![],
        IResult::Incomplete(_) => vec![],
    }
}

pub fn solve(input: &[u8]) -> (usize, usize) {
    let mut grid : LightGrid = HashMap::new();
    for x in 0..1000 {
        for y in 0..1000 {
            grid.insert((x,y), 0);
        }
    }
    println!("Created grid, starting instructions");
    for inst in parse_instructions(input).iter() {
        modify_light(&mut grid, inst.0, inst.1, inst.2);
    }
    let p1 = grid.values().filter(|v| { **v > 0 }).count();
    let p2 = grid.values().sum();
    (p1,p2)
}
