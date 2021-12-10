extern crate aoc;

use std::str::FromStr;

mod day7 {
    pub fn part_one(v: &Vec<u64>) -> u64 {
        let min = *(v.iter().min().unwrap());
        let max = *(v.iter().max().unwrap());

        (min..=max).map(|p| 
            v.iter()
             .map(|x| if x > &p { x - p } else { p - x })
             .sum()
        ).min().unwrap()
    }
    pub fn part_two(v: &Vec<u64>) -> u64 {
        let min = *(v.iter().min().unwrap());
        let max = *(v.iter().max().unwrap());

        (min..=max).map(|p| 
            v.iter()
             .map(|x| { 
                 let diff = if x > &p { x - p } else { p - x };
                 diff * (diff + 1) / 2
             })
             .sum()
        ).min().unwrap()
    }
}


fn main() {
    let contents = aoc::aoc::read_input(true);
    let _inputs = contents.split(',').map(|num_str| u64::from_str(num_str).unwrap()).collect();

    println!("minimum fuel value = {}.", day7::part_one(&_inputs));
    println!("minimum fuel value = {}.", day7::part_two(&_inputs));

    ()
}

