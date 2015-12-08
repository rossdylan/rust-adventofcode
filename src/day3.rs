use std::collections::HashMap;


fn get_and_update(hashmap: &mut HashMap<(i64,i64), i64>, xy: (i64, i64)) {
     let new = match hashmap.get_mut(&xy) {
        Some(val) => *val + 1,
        None => 1
     };
     hashmap.insert(xy, new);
}

pub fn robo_santa(string: &str) -> usize {
    let mut x : i64 = 0;
    let mut y : i64 = 0;
    let mut rx : i64 = 0;
    let mut ry : i64 = 0;
    let mut presents: HashMap<(i64, i64), i64> = HashMap::new();
    let mut robo : bool = false;
    for c in string.chars() {
        match (c,robo) {
            ('^', false) => { get_and_update(&mut presents, (x,y)); y += 1; get_and_update(&mut presents, (x,y)) },
            ('v', false) => { get_and_update(&mut presents, (x,y)); y -= 1; get_and_update(&mut presents, (x,y)) },
            ('>', false) => { get_and_update(&mut presents, (x,y)); x += 1; get_and_update(&mut presents, (x,y)) },
            ('<', false) => { get_and_update(&mut presents, (x,y)); x -= 1; get_and_update(&mut presents, (x,y)) }
            ('^', true) => { get_and_update(&mut presents, (rx,ry)); ry += 1; get_and_update(&mut presents, (rx,ry)) },
            ('v', true) => { get_and_update(&mut presents, (rx,ry)); ry -= 1; get_and_update(&mut presents, (rx,ry)) },
            ('>', true) => { get_and_update(&mut presents, (rx,ry)); rx += 1; get_and_update(&mut presents, (rx,ry)) },
            ('<', true) => { get_and_update(&mut presents, (rx,ry)); rx -= 1; get_and_update(&mut presents, (rx,ry)) }
            _ => ()
        };
        if robo == true {
            robo = false;
        }
        else {
            robo = true;
        }
    }
    presents.len()
}

pub fn handle_directions(string: &str) -> usize {
    let mut x : i64 = 0;
    let mut y : i64 = 0;
    let mut presents: HashMap<(i64, i64), i64> = HashMap::new();
    for c in string.chars() {
        match c {
            '^' => { get_and_update(&mut presents, (x,y)); y += 1; get_and_update(&mut presents, (x,y)) },
            'v' => { get_and_update(&mut presents, (x,y)); y -= 1; get_and_update(&mut presents, (x,y)) },
            '>' => { get_and_update(&mut presents, (x,y)); x += 1; get_and_update(&mut presents, (x,y)) },
            '<' => { get_and_update(&mut presents, (x,y)); x -= 1; get_and_update(&mut presents, (x,y)) }
            _ => ()
        };
    }
    presents.len()
}
