extern crate aoc;

use std::collections::HashMap;
use std::str::FromStr;

#[derive(Copy, Clone, Hash, Eq)]
struct BingoCellMetadata {
    col_index: usize,
    row_index: usize,
    marked: bool
}

// impl BingoCellMetadata {
//     fn new() -> Self {
//         Self {
//             col_index: 0usize,
//             row_index: 0usize,
//             marked: false
//         }
//     }
// }

impl PartialEq for BingoCellMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.col_index == other.col_index &&
        self.row_index == other.row_index &&
        self.marked == other.marked 
    }
}

#[derive(Clone)]
struct Bingo {
    table: HashMap<u32, BingoCellMetadata>,
    rows: Vec<Vec<bool>>,
    cols: Vec<Vec<bool>>,
    won: bool,
}

impl Bingo {
    fn new() -> Self {
        Self {
            table: HashMap::new(),
            rows: vec![vec![false; 5]; 5],
            cols: vec![vec![false; 5]; 5],
            won: false,
        }
    }
    fn new_from_lines(lines: Vec<&str>) -> Self {
        let mut result = Bingo::new();
        //println!("lines size is {}.", lines.clone().len());
        lines.iter()
            .map(|line| { 
                //println!("LIne is {}.", line);
                line.split_ascii_whitespace()
                    .map(|num_str| { 
                        //println!("num_str is {}", num_str); 
                        u32::from_str(num_str).unwrap() })
                    .collect::<Vec<u32>>() })
            .enumerate()
            .for_each(|(i, vn)| {
                vn.iter()
                .enumerate()
                .for_each(|(j, n)| {
                    result.table.insert(*n, 
                        BingoCellMetadata {
                            col_index: i,
                            row_index: j,
                            marked: false 
                    });
                });
            });

        result
    }

    fn mark(&mut self, n: u32) {
        let val = self.table.get(&n);

        if let Some(&metadata) = val {
            let i = metadata.col_index;
            let j = metadata.row_index;

            let mut new_metadata = metadata.clone();
            new_metadata.marked = true;

            self.table.insert(n, new_metadata);
            self.rows[j][i] = true;
            self.cols[i][j] = true;
            self.won = self._did_win();
        }


    }

    fn _did_win(&self) -> bool {
        self.rows.iter()
            .fold(false, |acc, x| acc ||  (x.iter().fold(true, |acc, x| acc && *x) )) ||
        self.cols.iter()
            .fold(false, |acc, x| acc ||  (x.iter().fold(true, |acc, x| acc && *x) )) 
    }

    fn sum_unchecked(&self) -> u64 {
        self.table.iter().filter(|(_, val)| !val.marked ).fold(0_u64, |acc, (&key, _)| { 
            acc + (key as u64)
        })
    }
}

struct Bingos {
    rounds: Vec<u32>,
    bingos: Vec<Bingo>,
    last_index: usize,
    first_winning_index: usize,
    last_winning_index: usize,
    first_multiplier_index: usize,
    last_multiplier_index: usize,
}

impl Bingos {

    fn new_from_lines(x: std::str::Lines) -> Self {
        let mut fst_line = x.clone().take(1);
        let rounds = fst_line.nth(0).unwrap().split(',').map(|x| u32::from_str(x).unwrap()).collect::<Vec<u32>>();
        let bingos = {
            let mut bingos: Vec<Bingo> = Vec::new();
            for i in (2..).step_by(6) {
                let new_line = x.clone().skip(i).take(5).collect::<Vec<&str>>();
                if new_line.is_empty() { break; }
                bingos.push(Bingo::new_from_lines(new_line));
            }
            bingos
        };
        Self {
            rounds: rounds,
            bingos: bingos,
            last_index: 0,
            first_winning_index: usize::MAX,
            last_winning_index: usize::MAX,
            first_multiplier_index: usize::MAX,
            last_multiplier_index: usize::MAX,
        }
    }

    fn step(&mut self) -> bool {
        if self.last_index > self.rounds.len() { 
            println!("Step is requested after rounds are finished.");
            return true;
        } 
        self.bingos.iter_mut().filter(|bingo| bingo.won == false).for_each(|bingo| bingo.mark(self.rounds[self.last_index]) );

        if self.first_winning_index == usize::MAX {
            let did_win = self.bingos.iter().fold(false, |acc, bingo| acc || bingo.won);
            if did_win {
                self.first_winning_index = self.bingos.iter().enumerate().filter(|(_, bingo)| bingo.won).take(1).map(|(i, _)| i).collect::<Vec<usize>>()[0];
                self.first_multiplier_index = self.last_index;
            }
        } else if self.last_winning_index == usize::MAX {
            let remaining_indices = self.bingos.iter().enumerate().filter(|(_, bingo)| bingo.won == false).map(|(i, _)| i).collect::<Vec<usize>>(); //fold(true, |acc, bingo| acc | bingo.won);
            if remaining_indices.len() == 1 {
                self.last_winning_index = remaining_indices[0];
            }
        } else if self.last_multiplier_index == usize::MAX {
            let did_win = self.bingos.iter().fold(true, |acc, bingo| acc && bingo.won);
            if did_win {
                self.last_multiplier_index = self.last_index;
            }
        }

        self.last_index += 1;
        self.last_multiplier_index != usize::MAX
    }

    fn first_multiplier(&self) -> u64 {
        self.rounds[self.first_multiplier_index] as u64
    }
    fn last_multiplier(&self) -> u64 {
        self.rounds[self.last_multiplier_index] as u64
    }
}

fn main() {
    let contents = aoc::aoc::read_input(true);
    
    let inputs = contents.lines();
    let mut bingos = Bingos::new_from_lines(inputs); 
    
    for _ in 0..(bingos.rounds.len()) {
        let break_loop = bingos.step();
        if break_loop {
            break;
        }
    }
    {
        let winning_index = bingos.first_winning_index;
        let sum_of_bingo = bingos.bingos[winning_index].sum_unchecked();
        
        println!("Bingo {}  won after {} steps with sum {}.", winning_index, bingos.first_multiplier_index, sum_of_bingo);
        println!("Result = {}.", sum_of_bingo * bingos.first_multiplier());
    }
    {
        let winning_index = bingos.last_winning_index;
        let sum_of_bingo = bingos.bingos[winning_index].sum_unchecked();
        
        println!("Bingo {}  won after {} steps with sum {}.", winning_index, bingos.last_multiplier_index, sum_of_bingo);
        println!("Result = {}.", sum_of_bingo * bingos.last_multiplier());
    }
    ()
}
