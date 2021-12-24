extern crate aoc;

mod day16 {
    #[derive(Clone, Copy, Debug)]
    pub struct Packet {
        v: u8,
        t: u8, 
        val: usize, 
    }
    impl Packet {
        fn new() -> Self {
            Self {
                v: 0,
                t: 0, 
                val: 0,
            }
        }
    }
    pub fn char_to_unsigned(c: char) -> u8 {
        match c {
            '0'    =>      0,
            '1'    =>      1,
            '2'    =>      2,
            '3'    =>      3,
            '4'    =>      4,
            '5'    =>      5,
            '6'    =>      6,
            '7'    =>      7,
            '8'    =>      8,
            '9'    =>      9,
            'A'    =>      10,
            'B'    =>      11,
            'C'    =>      12,
            'D'    =>      13,
            'E'    =>      14,
            'F'    =>      15,
             _     =>      u8::MAX,
        }
    }

    fn parse_literal(p: &Vec<u8>, index: usize) -> usize {
        let i = index >> 2;
        let s = index & 3;

        let mut result = 0;

        

        result 
    }
    fn parse_rec(p: &Vec<u8>, index: usize, packets: &mut Vec<Packet>) -> () {
        let i = index >> 2;
        let bits0 = p[i];
        let bits1 = p[i + 1];

        let mut packet = Packet::new();

        packet.v = bits0 >> 1;
        packet.t = (bits1 >> 2) | ((bits0 & 0b1) << 2);

        if packet.t == 4 {
            packet.val = parse_literal(p, index + 6);
        }

        packets.push(packet);
    }

    pub fn parse(p: &Vec<u8>) -> Vec<Packet> {
        let mut result = Vec::new();
        parse_rec(p, 0, &mut result);
        result
    }
}

fn main() {
    let contents = if cfg!(feature = "example") { aoc::aoc::read_example(true) } else { aoc::aoc::read_input(true) };
    let _inputs =  { 
        let result = Vec::new();

        let vals = contents.lines()
            .map(|line| 
                line.chars()
                    .map(|c| day16::char_to_unsigned(c))
                    .collect::<Vec<u8>>() )
            .collect::<Vec<Vec<u8>>>()[0].clone();
        
        vals.for_each(|c| result);
        
        result
    }

    println!("{:?}", _inputs);
    println!("{:?}", day16::parse(&_inputs));
    
    let part_one = 0;
    let part_two = 0;

    println!("Part One = {}.", part_one);
    println!("Part Two = {}.", part_two);

    ()
}
