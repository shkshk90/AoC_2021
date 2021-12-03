extern crate aoc;

#[derive(Clone)]
struct PairSum {
    sum_0: i64,
    sum_1: i64,
}

impl PairSum {
    fn new_with_bit(bit_char_opt: Option<char>) -> Self {
        match bit_char_opt {
            Some(bit_char) =>
                if bit_char == '0' {
                    Self {
                        sum_0: 1i64, 
                        sum_1: 0i64,
                    }
                } else if bit_char == '1' {
                    Self {
                        sum_0: 0i64, 
                        sum_1: 1i64,
                    }
                } 
                else {
                    Self {
                        sum_0: 0i64, 
                        sum_1: 0i64,
                    }
                },
            _ =>  Self {
                sum_0: 0i64, 
                sum_1: 0i64,
            }
        }
    }
    fn add(&self, bit_char: char) -> PairSum {
        if bit_char == '0' {
            PairSum {
                sum_0: self.sum_0 + 1,
                sum_1: self.sum_1,
            }
        } else if bit_char == '1' {
            PairSum {
                sum_0: self.sum_0,
                sum_1: self.sum_1 + 1,
            }
        } 
        else {
            PairSum {
                sum_0: self.sum_0,
                sum_1: self.sum_1,
            }
        } 
    }
    fn most_common_bit(&self) -> u64 {
        if self.sum_0 > self.sum_1 { 0u64 } else { 1u64 } 
    }
    fn least_common_bit(&self) -> u64 {
        self.most_common_bit() ^ 1u64
    }
}

struct SumsArray {
    sums: std::vec::Vec<PairSum>,
}

impl SumsArray {
    fn new() -> Self {
        Self { 
            sums: Vec::new()
        }
    }
    
    fn add(&self, input: &str) -> SumsArray {
        let sums_iter = self.sums.iter();
        let result = {
            if self.sums.is_empty() {
                input.chars().map(|x| PairSum::new_with_bit(Some(x))).collect()
            } else {
                sums_iter.zip(input.chars()).map(|(sum, bit_char)| {
                    sum.add(bit_char)
                }).collect()
            }
        };

        SumsArray {
            sums: result
        }
    }

    fn gamma(&self) -> u64 {
        self.sums.iter().fold(0u64, |acc, pair_sum| (acc << 1) | pair_sum.most_common_bit() )
    }
    fn epsilon(&self) -> u64 {
        self.sums.iter().fold(0u64, |acc, pair_sum| (acc << 1) | pair_sum.least_common_bit() )
    }

    fn part_one(&self) -> u64 {
        self.gamma() * self.epsilon()
    }
}



fn main() {
    let contents = aoc::aoc::read_input(true);
    
    let inputs = contents.lines();
    let final_sum = inputs.fold(SumsArray::new(), |acc, x| { 
        acc.add(x)
    });

    println!("Result of part one = {}.", final_sum.part_one());
    
    let bit_count = final_sum.sums.len();

    let mut oxygen = contents.lines().collect::<Vec<&str>>();
    let mut co_two = contents.lines().collect::<Vec<&str>>();

    for i in 0..bit_count {
        if oxygen.len() > 1 {
            let oxygen_pair = oxygen.iter().fold(PairSum::new_with_bit(None), |acc, x| { 
                acc.add(x.chars().nth(i).unwrap())
            });
            let most_common_bit = if oxygen_pair.most_common_bit() == 1 { '1' } else { '0' };
            oxygen = oxygen.into_iter().filter(|x| x.chars().nth(i).unwrap() == most_common_bit ).collect(); 
        }
        if co_two.len() > 1 {
            let co_two_pair = co_two.iter().fold(PairSum::new_with_bit(None), |acc, x| { 
                acc.add(x.chars().nth(i).unwrap())
            });
            let least_common_bit = if co_two_pair.least_common_bit() == 1 { '1' } else { '0' };
            co_two = co_two.into_iter().filter(|x| x.chars().nth(i).unwrap() == least_common_bit ).collect();
        }
    }

    let oxygen_value = u32::from_str_radix(oxygen[0], 2).unwrap();
    let co_two_value = u32::from_str_radix(co_two[0], 2).unwrap();

    println!("O2: {},  CO2: {}.", oxygen_value, co_two_value);
    println!("Result of part two = {}.", oxygen_value * co_two_value);
    
    ()
}
