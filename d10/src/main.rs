extern crate aoc;

mod day10 {
    fn is_good_pair(c0: char, c1: char) -> bool {
        c0 == '(' && c1 == ')' || 
        c0 == '[' && c1 == ']' ||
        c0 == '{' && c1 == '}' ||
        c0 == '<' && c1 == '>'
    }
    pub fn is_a_corrupted_line(v: &Vec<char>) -> (bool, char, String) {
        let mut stack: Vec<char> = Vec::new();

        let is_corrupted = v.iter().fold('_', |acc, c| {
            if acc != '_' { return acc }
            
            match *c {
                '(' | '{' | '[' | '<'  => stack.push(*c),
                ')' | '}' | ']' | '>'  => {
                    if stack.len() == 0 { return *c; }
                    let opening_char = stack.pop().unwrap();
                    if is_good_pair(opening_char, *c) { return acc }
                    else { return *c}
                    
                },
                _ => () 
            };
            acc
        });

        (is_corrupted != '_', is_corrupted, stack.iter().collect::<String>())
    }

    pub fn score(c: char) -> u64 {
        match c {
            ')' => 3_u64,
            ']' => 57_u64,
            '}' => 1197_u64,
            '>' => 25137_u64,
            _   => 0_u64,
        }
    }

    pub fn auto_complete(s: &String) -> String {
        s.chars().map(|c| {
            match c {
                '(' => ')',
                '{' => '}',
                '[' => ']',
                '<' => '>',
                _   =>  c
            }
        }).rev().collect::<String>()
    }

    pub fn score2(c: char) -> u64 {
        match c {
            ')' => 1_u64,
            ']' => 2_u64,
            '}' => 3_u64,
            '>' => 4_u64,
            _   => 0_u64,
        }
    }

    pub fn auto_complete_score(s: &String) -> u64 {
        s.chars().fold(0_u64, |acc, c| (acc * 5) + score2(c))
    }
}

fn main() {
    let contents  = aoc::aoc::read_input(true);
    let _inputs   = contents.lines().map(|line| line.chars().collect::<Vec<char>>() ).collect::<Vec<Vec<char>>>();
    let processed = _inputs.iter().map(|v| day10::is_a_corrupted_line(v) );
    let corrupted = processed.clone().filter(|(is_corrupted, _, _)| *is_corrupted).collect::<Vec<(bool, char, String)>>();
    let result_1  = corrupted.iter().fold(0, |acc, (_, c, _)| acc + day10::score(*c));

    let auto_complte_strs = processed.clone()
        .filter(|(is_corrupted, _, _)| *is_corrupted == false)
        .map(|(_, _, s)| day10::auto_complete(&s) )
        .collect::<Vec<String>>();
    let mut result_2 = auto_complte_strs.iter().map(|s| day10::auto_complete_score(&s)).collect::<Vec<u64>>();
    result_2.sort();
    
    println!("Part 1 = {}", result_1);
    println!("Part 2 = {}", result_2[result_2.len() / 2]);

    ()
}

