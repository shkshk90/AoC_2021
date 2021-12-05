
extern crate aoc;

use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone)]
struct Test {
    x: u32
}

impl Test {
    fn new() -> Self {
        Self {
            x: 0_u32
        }
    }
}


fn main() {
    let contents = aoc::aoc::read_input(true);
    let _inputs = contents.lines();
    
    ()
}

