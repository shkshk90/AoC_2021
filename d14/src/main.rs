extern crate aoc;

mod day14 {
    const A_US: usize = 'A' as usize;
    const Z_US: usize = 'Z' as usize;
    const VEC_LEN: usize = Z_US + 1;
    const INDEX_SHIFT: usize = 5;
    const MASK: usize = 0b11111;
    const DUMMY_US: usize = usize::MAX;

    fn char_to_index(c: char) -> usize {
        (c as usize) - A_US
    }
    fn index_to_char(n: usize) -> char {
        ((n & MASK) + A_US) as u8 as char
    }
    pub fn pair_to_index(cc: &(char, char)) -> usize {
        let i = char_to_index(cc.0);
        let j = char_to_index(cc.1);

        (i << INDEX_SHIFT) | j

    }

    pub fn index_to_pair(n: usize) -> (char, char) {
        (
            index_to_char(n >> INDEX_SHIFT), 
            index_to_char(n)
        )
    }
    pub fn make_1d_map(v: &Vec<(&str, &str)>) -> Vec<(usize, usize)> {
        let mut result = vec![(DUMMY_US, DUMMY_US); (VEC_LEN << INDEX_SHIFT) | VEC_LEN];

        v.iter().for_each(|(s0, s1)| {
            let chars = s0.chars();

            let lvl0 = chars.clone().nth(0).unwrap();
            let lvl1 = chars.clone().nth(1).unwrap();

            let poly = s1.chars().nth(0).unwrap();

            let pair_index_0 = pair_to_index(&(lvl0, poly));
            let pair_index_1 = pair_to_index(&(poly, lvl1));

            result[pair_to_index(&(lvl0, lvl1))] = (pair_index_0, pair_index_1);

        });

        result
    }
    fn solve_pair_step(
        poly_map_1d: &Vec<(usize, usize)>, 
        result: &mut Vec<usize>
    ) {
        let old_result = result.clone();

        old_result.iter()
            .enumerate()
            .filter(|(_, &n)| n > 0)
            .for_each(|(i, p)| {

            let (i0, i1) = poly_map_1d[i];
            
            result[i]  -= p;
            result[i0] += p;
            result[i1] += p;
        });
    }
    pub fn solve(
        poly_map_1d: &Vec<(usize, usize)>, 
        chain_pairs: &Vec<(char, char)>,
        count:usize) -> Vec<usize> {

        let mut result = vec![0; VEC_LEN * VEC_LEN];

        chain_pairs.iter().for_each(|p| {
            result[pair_to_index(p)] += 1;
        });

        (0..count).for_each(|_| 
            solve_pair_step(poly_map_1d, &mut result)
        );

        result
    }
    pub fn pairs_to_letters(arr: &Vec<usize>, first_char: char) -> Vec<usize> {
        let mut vals: Vec<usize> = vec![0; VEC_LEN];

        vals[char_to_index(first_char)] = 1;
        arr.iter()
            .enumerate()
            .filter(|(_, n)| **n > 0)
            .for_each(|(i, &n)| {
                let (_, i1) = index_to_pair(i);

                //vals[char_to_index(i0)] += n;
                vals[char_to_index(i1)] += n;

                ()
            });

        vals
    } 
    fn min_max(arr: &Vec<usize>) -> (usize, usize) {
        
        let min = arr.iter().filter(|n| **n > 0).min().unwrap();
        let max = arr.iter().max().unwrap();

        (*min, *max)
    }
    pub fn solution(arr: &Vec<usize>, first_char: char) -> usize {
        let letters = pairs_to_letters(arr, first_char);
        let (min, max) = min_max(&letters);
        
        max - min
    }
}

fn main() {
    let contents = if cfg!(feature = "example") { aoc::aoc::read_example(true) } else { aoc::aoc::read_input(true) };
    let _inputs = contents.lines();
    let _pairs = &_inputs.clone()
        .take(1)
        .map(|line| {
            let fst = line.chars();
            let snd = line.chars().skip(1);

            fst.zip(snd)
                .map(|(a, b)| (a, b))
                .collect::<Vec<(char, char)>>()
        })
        .collect::<Vec<Vec<(char, char)>>>()[0];
    let _first_char = *(&_inputs.clone()
        .take(1)
        .map(|line| line.chars().nth(0).unwrap())
        .collect::<Vec<char>>()[0]);
    let _polymerization = _inputs.clone()
        .skip(2)
        .map(|line| line.split(' ').collect::<Vec<&str>>() )
        .map(|vs|   (vs[0], vs[2]) )
        .collect::<Vec<(&str, &str)>>();
    let _map = day14::make_1d_map(&_polymerization); 


    let result_1 = day14::solve(&_map, &_pairs, 10);
    let result_2 = day14::solve(&_map, &_pairs, 40);

    let part_one = day14::solution(&result_1, _first_char);
    let part_two = day14::solution(&result_2, _first_char);

    println!("Part One = {}.", part_one);
    println!("Part Two = {}.", part_two);

    ()
}
