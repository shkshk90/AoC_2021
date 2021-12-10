
extern crate aoc;

use std::str::FromStr;

mod day6 {
    const CYCLE_START: usize = 6;
    const LIFE_START:  usize = 8;

    fn new_fish_at_day(d: usize) -> Vec<u64> {
        let mut result = vec![0; LIFE_START + 1];
        result[d] = 1;

        result
    }
 
    fn shift_vec(v: &mut Vec<u64>) {
        let count_to_add = v[0];

        for i in 1..v.len() {
            v[i - 1] = v[i];
        }

        v[CYCLE_START] += count_to_add;
        v[LIFE_START]   = count_to_add;
    }

    pub fn create_cache(d: u64) -> Vec<u64> {
        let mut fish_vec = new_fish_at_day(CYCLE_START);

        for _ in 0..(d - 1) {
            shift_vec(&mut fish_vec);
        }

        (0..=CYCLE_START).map(|_| {
            shift_vec(&mut fish_vec);
            fish_vec.iter().sum()
        }).collect::<Vec<u64>>().into_iter().rev().collect()
    }
}

fn main() {
    let contents = aoc::aoc::read_input(true);
    let _inputs = contents.split(',').map(|num_str| usize::from_str(num_str).unwrap());

    {
        let cache = day6::create_cache(80);
        let result: u64 = _inputs.clone().map(|x| cache[x]).sum();

        println!("children after  80 days = {}.", result);
    }
    {
        let cache = day6::create_cache(256);
        let result: u64 = _inputs.clone().map(|x| cache[x]).sum();

        println!("children after 256 days = {}.", result);
    }

    ()
}

