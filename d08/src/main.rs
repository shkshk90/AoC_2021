
extern crate aoc;

mod day8 {
    use std::collections::HashMap;

    fn pattern_to_digit(mask: u8) -> u8 {
        match mask {
            0b1110111 => 0,
            0b0100100 => 1,
            0b1011101 => 2,
            0b1101101 => 3,
            0b0101110 => 4,
            0b1101011 => 5,
            0b1111011 => 6,
            0b0100101 => 7,
            0b1111111 => 8,
            0b1101111 => 9,
            _ => 255
        }
    }
    fn to_u8(c: char) -> u8 {
        match c {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            _ => 255,
        }
    }
    pub fn to_mask(s: &str) -> u8 {
        s.chars().map(|c| {
            let shift = to_u8(c);
            1_u8 << shift
        }).fold(0_u8, |acc, mask| acc | mask)
    }
    pub fn to_pattern(s: usize) -> u8 {
        match s {
            2 => to_mask("cf"), // 1
            4 => to_mask("bdcf"), // 4
            3 => to_mask("acf"), // 7
            7 => to_mask("abcdefg"), // 8
            _ => 0_u8
        }
    }
    pub fn find_pattern(vs: &Vec<&str>) -> Vec<u8> {
        let mut result = vec![0b11111111; 7];

        vs.iter()
            .filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)
            .for_each(|s|  {
                let s_len = s.len();
                s.chars().for_each(|c| result[to_u8(c) as usize] = result[to_u8(c) as usize] & to_pattern(s_len))
            } );

        result
    }
    pub fn match_str(s: &str, p: &Vec<u8>) -> u8 {
        if s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7 {
            pattern_to_digit( 
                s.chars().fold(0_u8, |acc, c| acc | p[to_u8(c) as usize]))
        } else {
            255
        }
    }

    pub fn make_number_map<'a>(vp: &Vec<&'a str>) -> HashMap<u8, u8> {
        let mut map = vec![(0u8, ""); 10];
        
        vp.iter().for_each(|s| match s.len() {
            2 => map[1] = (to_mask(s), s),
            3 => map[7] = (to_mask(s), s),
            4 => map[4] = (to_mask(s), s),
            7 => map[8] = (to_mask(s), s),
            _ => ()
        });
        vp.iter()
            .filter(|s| s.len() == 6)
            .for_each(|x| {
                let mask = to_mask(x);
                let result = mask ^ map[4].0;
                match result.count_ones() {
                    2 => map[9] = (mask, x), 
                    4 => if map[7].0 & mask == map[7].0 { map[0] = (mask, x) } else { map[6] = (mask, x) } ,
                    _ => ()
                }
            }
        );
        vp.iter()
            .filter(|s| s.len() == 5)
            .for_each(|x| {
                let mask = to_mask(x);
                let result = mask & map[6].0;
                match result.count_ones() {
                    5 => map[5] = (mask, x), 
                    4 => if map[1].0 & mask == map[1].0 { map[3] = (mask, x) } else { map[2] = (mask, x) }, 
                    _ => ()
                }
            }
        );

        let num_map = map.iter().map(|(n, _)| *n).collect::<Vec<u8>>();
        let mut result = HashMap::new();

        num_map.iter().enumerate().for_each(|(i, n)| { result.insert(*n, i as u8); () } );
        //println!("Map: {:?}", result);
        result
    }

    pub fn str_to_num(s: &str, m: &HashMap<u8, u8>) -> u8 {
        let mask = to_mask(s);
        //println!("str = {}, mask = {:#08b}, {}", s, mask, mask);
        *m.get(&mask).unwrap()
    }
}


fn main() {
    let contents = aoc::aoc::read_input(true);
    let _inputs = contents.lines().map(|x| {
        let parts = x.split('|').collect::<Vec<&str>>();
        (parts[0], parts[1])
    }).map(|(a, b)| (a.split(' ').collect::<Vec<&str>>(), b.split(' ').collect::<Vec<&str>>())).collect::<Vec<(Vec<&str>, Vec<&str>)>>();

    let vec_result_1 = _inputs.iter().map(|(vp, vs)| { 
        let v = day8::find_pattern(&vp).clone();
        vs.iter().map(move |s| day8::match_str(s, &v)).collect::<Vec<u8>>()
    }).collect::<Vec<Vec<u8>>>();

    let result_1 = vec_result_1.iter()
        .fold(0_u32, |acc, v| acc + v.iter()
            .fold(0_u32, |acc, &x| acc + if x == 255 { 0 } else { 1 }) );
    
    println!("part one result = {:?}", result_1);

    let _test_example = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    let _inputs_2 = contents.lines().map(|x| {
        let parts = x.split('|').collect::<Vec<&str>>();
        (parts[0], parts[1])
    }).map(|(a, b)| (a.split(' ').collect::<Vec<&str>>(), b.split(' ').collect::<Vec<&str>>())).collect::<Vec<(Vec<&str>, Vec<&str>)>>();
    
    let outputs = _inputs_2.iter()
        .map(|(vp, vs)| (day8::make_number_map(&vp), vs))
        .map(|(number_map, vs)| vs.iter().skip(1).map(|s| day8::str_to_num(s, &number_map)).fold(0_u64, |acc, n| (acc * 10) + (n as u64)) ).collect::<Vec<u64>>();
    let final_result: u64 = outputs.iter().sum();

    println!("{:?}", outputs);
    println!("{:?}", final_result);



    ()
}

