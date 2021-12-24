extern crate aoc;

mod day12 {
    use std::collections::HashMap;
    use std::collections::HashSet;

    const START_VAL: u16 = 1_u16;
    const END_VAL: u16 = !0_u16;

    fn char_to_num(c: char) -> u8 {
        match c {
            'a'     =>      3,
            'b'     =>      4,
            'c'     =>      5,
            'd'     =>      6,
            'e'     =>      7,
            'f'     =>      8,
            'g'     =>      9,
            'h'     =>      10,
            'i'     =>      11,
            'j'     =>      12,
            'k'     =>      13,
            'l'     =>      14,
            'm'     =>      15,
            'n'     =>      16,
            'o'     =>      17,
            'p'     =>      18,
            'q'     =>      19,
            'r'     =>      20,
            's'     =>      21,
            't'     =>      22,
            'u'     =>      23,
            'v'     =>      24,
            'w'     =>      25,
            'x'     =>      26,
            'y'     =>      27,
            'z'     =>      28,
            'A'     =>      33,
            'B'     =>      34,
            'C'     =>      35,
            'D'     =>      36,
            'E'     =>      37,
            'F'     =>      38,
            'G'     =>      39,
            'H'     =>      40,
            'I'     =>      41,
            'J'     =>      42,
            'K'     =>      43,
            'L'     =>      44,
            'M'     =>      45,
            'N'     =>      46,
            'O'     =>      47,
            'P'     =>      48,
            'Q'     =>      49,
            'R'     =>      50,
            'S'     =>      51,
            'T'     =>      52,
            'U'     =>      53,
            'V'     =>      54,
            'W'     =>      55,
            'X'     =>      56,
            'Y'     =>      57,
            'Z'     =>      58,
             _      =>      0
        }
    }
    fn num_to_char(n: u16) -> char {
        match n {
            3       =>      'a',
            4       =>      'b',
            5       =>      'c',
            6       =>      'd',
            7       =>      'e',
            8       =>      'f',
            9       =>      'g',
            10      =>      'h',
            11      =>      'i',
            12      =>      'j',
            13      =>      'k',
            14      =>      'l',
            15      =>      'm',
            16      =>      'n',
            17      =>      'o',
            18      =>      'p',
            19      =>      'q',
            20      =>      'r',
            21      =>      's',
            22      =>      't',
            23      =>      'u',
            24      =>      'v',
            25      =>      'w',
            26      =>      'x',
            27      =>      'y',
            28      =>      'z',
            33      =>      'A',
            34      =>      'B',
            35      =>      'C',
            36      =>      'D',
            37      =>      'E',
            38      =>      'F',
            39      =>      'G',
            40      =>      'H',
            41      =>      'I',
            42      =>      'J',
            43      =>      'K',
            44      =>      'L',
            45      =>      'M',
            46      =>      'N',
            47      =>      'O',
            48      =>      'P',
            49      =>      'Q',
            50      =>      'R',
            51      =>      'S',
            52      =>      'T',
            53      =>      'U',
            54      =>      'V',
            55      =>      'W',
            56      =>      'X',
            57      =>      'Y',
            58      =>      'Z',
            _       =>      '-',
        }
    }
    fn is_small_cave(cave: u16) -> bool {
        (cave & 0x20) == 0
    }

    pub fn is_large_cave(cave: u16) -> bool {
        !is_small_cave(cave)
    }
    
    pub fn hash_chars(cave: &str) -> u16 {
        if cave == "start" { return START_VAL }
        if cave == "end"   { return END_VAL }
        let mut chars = cave.chars();

        let c0 = chars.next().unwrap();
        let c1 = chars.next().unwrap();

        let n0 = char_to_num(c0) as u16;
        let n1 = char_to_num(c1) as u16;

        (n0 << 8) | n1
    }
    pub fn to_map(v: &Vec<(u16, u16)>) -> HashMap<u16, Vec<u16>> {
        let mut result = HashMap::new();


        v.iter().for_each(|(a, b)| {
            if *b != START_VAL && *a != END_VAL { 

                let va_opt = result.get_mut(a);
                
                if va_opt.is_none() {
                    result.insert(*a, vec![*b]);
                } else {
                    va_opt.unwrap().push(*b);
                }
            }
            if *a != START_VAL && *b != END_VAL{

                let vb_opt = result.get_mut(b);
                
                if vb_opt.is_none() {
                    result.insert(*b, vec![*a]);
                } else {
                    vb_opt.unwrap().push(*a);
                }
            }
        });

        

        result
    }
    pub fn to_map_s<'a>(v: &Vec<(&'a str, &'a str)>) -> HashMap<&'a str, Vec<&'a str>> {
        let mut result = HashMap::new();


        v.iter().for_each(|(a, b)| {
            {
                if *b != "start" && *a != "end" { 

                    let va_opt = result.get_mut(a);
                    
                    if va_opt.is_none() {
                        result.insert(*a, vec![*b]);
                    } else {
                        va_opt.unwrap().push(*b);
                    }
                }
            }
            {
                if *a != "start" && *b != "end"
                {
                    let vb_opt = result.get_mut(b);
                    
                    if vb_opt.is_none() {
                        result.insert(*b, vec![*a]);
                    } else {
                        vb_opt.unwrap().push(*a);
                    }
                }
            }
        });

        

        result
    }

    fn build_path(
        hm: &HashMap<u16, Vec<u16>>,
        result: &mut Vec<Vec<u16>>,
        hs: HashSet<u16>, 
        current_path: Vec<u16>, 
        current_key: u16,
        current_index: usize) {

        let possible_paths = hm.get(&current_key).unwrap();
        let next_node_to_append = possible_paths[current_index];

        if hs.contains(&next_node_to_append) { return }

        let mut path = current_path.clone();
        path.push(next_node_to_append);

        if next_node_to_append == END_VAL {
            result.push(path);
            return
        }

        let next_possible_paths = hm.get(&next_node_to_append).unwrap();
        let mut cache = hs.clone();

        if is_small_cave(next_node_to_append) {
            cache.insert(next_node_to_append);
        }

        for (i, _) in next_possible_paths.iter().enumerate() {
            build_path(hm, result, cache.clone(), path.clone(), next_node_to_append, i);
        }
    }

    pub fn build_paths(hm: &HashMap<u16, Vec<u16>>) -> Vec<Vec<u16>> {
        let mut result = Vec::new();

        let start_vec = hm.get(&START_VAL).unwrap();

        for (i, _) in start_vec.iter().enumerate() {
            build_path(hm, &mut result, HashSet::new(), vec![START_VAL], START_VAL, i);
        }

        result
    }

    fn build_path_II(
        hm: &HashMap<u16, Vec<u16>>,
        result: &mut Vec<Vec<u16>>,
        hs: HashMap<u16, u8>, 
        has_passed_twice: bool,
        current_path: Vec<u16>, 
        current_key: u16,
        current_index: usize) {

        let possible_paths = hm.get(&current_key).unwrap();
        let next_node_to_append = possible_paths[current_index];
        
        if hs.contains_key(&next_node_to_append) && has_passed_twice {
            return
        }
        
        
        let mut path = current_path.clone();
        path.push(next_node_to_append);

        if next_node_to_append == END_VAL {
            result.push(path);
            return
        }

        let next_possible_paths = hm.get(&next_node_to_append).unwrap();
        let mut cache = hs.clone();
        let mut is_passed_twice = has_passed_twice;

        if is_small_cave(next_node_to_append) {
            let cave_was_maybe_passed = cache.get_mut(&next_node_to_append);
            if cave_was_maybe_passed.is_none() {
                cache.insert(next_node_to_append, 1u8);
            } else {
                *cave_was_maybe_passed.unwrap() += 1;
                is_passed_twice = true;
            }
        }

        for (i, _) in next_possible_paths.iter().enumerate() {
            build_path_II(hm, result, cache.clone(), is_passed_twice, path.clone(), next_node_to_append, i);
        }
    }

    pub fn build_paths_II(hm: &HashMap<u16, Vec<u16>>) -> Vec<Vec<u16>> {
        let mut result = Vec::new();

        let start_vec = hm.get(&START_VAL).unwrap();

        for (i, _) in start_vec.iter().enumerate() {
            build_path_II(hm, &mut result, HashMap::new(), false, vec![START_VAL], START_VAL, i);
        }

        result
    }
}

fn main() {
    let contents = if cfg!(feature = "example") { aoc::aoc::read_example(true) } else { aoc::aoc::read_input(true) };
    let _inputs = contents.lines()
        .map(|s| s.split('-')
            .map(|ss| day12::hash_chars(ss) )
            .collect::<Vec<u16>>() )
        .map(|v| (v[0], v[1]))
        .collect::<Vec<(u16, u16)>>();
    let _inputs_s = contents.lines()
        .map(|s| s.split('-')
            .collect::<Vec<&str>>() )
        .map(|v| (v[0], v[1]))
        .collect::<Vec<(&str, &str)>>();

    let hm = day12::to_map(&_inputs);
    // let hms = day12::to_map_s(&_inputs_s);
    // println!("{:?}", hms);

    let final_res = day12::build_paths(&hm);
    //println!("{:?}", final_res);
    println!("{}", final_res.len());

    let part_two = day12::build_paths_II(&hm);
    
    println!("Part One = {}.", final_res.len());
    println!("Part Two = {}.", part_two.len());

    ()
}
