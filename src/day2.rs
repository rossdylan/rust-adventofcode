use nom::IResult;
use nom::digit;
use nom::IResult::*;

use std::str;
use std::str::FromStr;
use std::cmp::min;

pub type Dimensions = (i64, i64, i64);


named!(number<i64>,
    map_res!(
        map_res!(
            digit,
            str::from_utf8
        ),
        FromStr::from_str
    )
);

named!(xyz<&[u8], Dimensions>,
    chain!(
        x: number ~
        tag!("x") ~
        y: number ~
        tag!("x") ~
        z: number ~
        tag!("\n") ,

        || (x, y, z)
    )
);

named!(all_xyz<&[u8], Vec<Dimensions> >, many0!(xyz));


/// Take a string and parse it into a vector of Dimensions.
/// Returns an optional in order to handle the failure cases
pub fn parse_dimensions(input: &[u8]) -> Option<Vec<Dimensions>> {
    match all_xyz(input) {
        IResult::Done(_, res) => Some(res),
        IResult::Error(_) => None,
        IResult::Incomplete(_) => None,
    }
}

pub fn paper_required(input : &[u8]) -> i64 {
    let dims : Vec<Dimensions> = parse_dimensions(input).unwrap();
    dims.iter().map(
        |&(l, w, h)| { (2*l*w) + (2*w*h) + (2*l*h) + min(min(l*w, w*h), l*h)}
        ).sum()
}

pub fn ribbon_required(input : &[u8]) -> i64 {
    let dims : Vec<Dimensions> = parse_dimensions(input).unwrap();
    fn vol (l : i64, w : i64, h : i64) -> i64 {
        l*w*h
    }

    fn perim(l : i64, w : i64, h : i64) -> i64 {
        min(min((2*l)+(2*w), (2*l)+(2*h)), (2*h)+(2*w))
    }

    dims.iter().map(|&(l, w, h)| { vol(l, w, h) + perim(l, w, h)}).sum()
}

#[cfg(test)]
mod tests {
    extern crate test;
    use nom::{IResult, digit};
    #[test]
    fn test_xyz() {
        let res : Vec<super::Dimensions> = super::parse_dimensions("10x11x12").unwrap();
        let real : Vec<super::Dimensions> = vec![(10, 11, 12)];
        assert_eq!(res, real);
    }
}
