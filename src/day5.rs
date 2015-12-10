pub fn is_good(string: &str) -> bool {
    let mut last : char = '.';
    let mut doubles : bool = false;
    let mut vcount : i64 = 0;
    let mut bads : bool = false;
    for c in string.chars() {
        match (c,last) {
            ('a',_) => vcount += 1,
            ('e',_) => vcount += 1,
            ('i',_) => vcount += 1,
            ('o',_) => vcount += 1,
            ('u',_) => vcount += 1,
            (_,_) => ()
        };
        match (c,last) {
            ('b','a') => bads = true,
            ('d','c') => bads = true,
            ('q','p') => bads = true,
            ('y','x') => bads = true,
            (a,b) => if a == b {doubles = true;}
        }
        if bads {
            break;
        }
        last = c;
    }
    return !bads && doubles && (vcount > 2);
}

pub fn solve(input: &str) -> usize {
    let mut count = 0;
    let mut total = 0;
    for line in input.lines() {
        if is_good(line) == true{
            println!("{} is good", line);
            count += 1;
        }
        total += 1;
    }
    println!("Total: {}", total);
    count
}
