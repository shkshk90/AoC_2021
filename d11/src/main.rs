extern crate aoc;

mod day11 {
    const FLASH_MASK: u32 = 1_u32 << 31;
    const FLASH_CLEAN_MASK: u32 = !FLASH_MASK;
    const PADDING_MASK: u32 = FLASH_MASK;

    fn flash(vvn: &mut Vec<Vec<u32>>, i: usize, j: usize) -> () {
        if vvn[i][j] & FLASH_MASK != 0 { return }
        let indices: [(usize, usize); 8] = [
            (i - 1, j - 1),
            (i - 1, j),
            (i - 1, j + 1),
            (i,     j - 1),
            (i,     j + 1),
            (i + 1, j - 1),
            (i + 1, j),
            (i + 1, j + 1),
        ]; 

        vvn[i][j] = FLASH_MASK;

        for (ri, rj) in indices.iter() {
            let i = *ri;
            let j = *rj;

            if vvn[i][j] & FLASH_MASK != 0 { 
                continue  // flashed already
            }  
            
            vvn[i][j] += 1;
            if vvn[i][j] > 9 {
                flash(vvn, i, j);
            }
        }
    }
    fn energy_step(vvn: &mut Vec<Vec<u32>>) -> u64 {
        let mut flashes = 0_u64;
        let ie = vvn.len() - 1;
        for i in 1..ie {
            let je = vvn[i].len() - 1;
            for j in 1..je {
                vvn[i][j] += 1;
                if vvn[i][j] > 9 {
                    flash(vvn, i, j);
                }
            }
        }

        for i in 1..ie {
            let je = vvn[i].len() - 1;
            for j in 1..je {
                if vvn[i][j] & FLASH_MASK == 0 { continue }

                flashes += 1;
                vvn[i][j] = 0;
            }
        }

        flashes
    }

    fn add_padding(vvn: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
        let mut result = Vec::new();

        result.push(vec![PADDING_MASK; vvn.len() + 2]);
        for i in 0..vvn.len() {
            result.push(Vec::new());
            result[i + 1].push(PADDING_MASK);
            for j in 0..vvn[i].len() {
                result[i + 1].push(vvn[i][j]);
            }
            result[i + 1].push(PADDING_MASK);
        }
        result.push(vec![PADDING_MASK; vvn.len() + 2]);

        result
    }

    fn print_padded(vvn: &Vec<Vec<u32>>) -> () {
        let ie = vvn.len() - 1;
        print!("---");
        for i in 1..ie {
            println!("");
            let je = vvn[i].len() - 1;
            for j in 1..je {
                print!("{}, ", vvn[i][j] & FLASH_CLEAN_MASK);
            }
        }
        println!("");
        println!("---");
    }

    pub fn count_flashes(octopuses: &Vec<Vec<u32>>,  days: u32) -> u64 {
        let mut flashes = 0_u64;
        let mut padded_oct = add_padding(&octopuses);

        (0..days).for_each(|_| 
            flashes += energy_step(&mut padded_oct)
        );

        flashes
    } 

    pub fn find_synchronized_flashes(octopuses: &Vec<Vec<u32>>) -> usize {
        let total_count = (octopuses.len() * octopuses[0].len()) as u64;
        let mut result = 0;
        let mut padded_oct = add_padding(&octopuses);

        for i in 0.. {
            let flashes = energy_step(&mut padded_oct);
            if flashes == total_count {
                result = i + 1;
                break;
            }
        }

        result
    }
}

fn main() {
    let contents = if cfg!(feature = "example") { aoc::aoc::read_example(true) } else { aoc::aoc::read_input(true) };
    let _inputs = contents.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap() ).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();
    
    let part_one = day11::count_flashes(&_inputs, 100);
    let part_two = day11::find_synchronized_flashes(&_inputs);

    println!("Part One = {}.", part_one);
    println!("Part Two = {}.", part_two);

    ()
}

