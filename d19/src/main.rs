extern crate aoc;

mod day19 {
}

fn main() {
    let contents = if cfg!(feature = "example") { aoc::aoc::read_example(true) } else { aoc::aoc::read_input(true) };
    let _inputs = contents.lines();
    
    let part_one = 0;
    let part_two = 0;

    println!("Part One = {}.", part_one);
    println!("Part Two = {}.", part_two);

    ()
}
