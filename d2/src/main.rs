use std::str::FromStr;

extern crate aoc;

struct SubmarinePosition {
    horizontal_pos: i32,
    depth: i32,
    aim: i32,
}

impl SubmarinePosition {
    fn new() -> Self {
        Self { 
            horizontal_pos: 0i32, 
            depth: 0i32,
            aim: 0i32
        }
    }
    fn add_part_one(&self, direction: &str, count: i32) -> SubmarinePosition {
        
        if direction == "up" {  
            SubmarinePosition { 
                horizontal_pos: self.horizontal_pos, 
                depth: self.depth - count,
                aim: self.aim
            }  
        }
        else if direction == "down" {  
            SubmarinePosition { 
                horizontal_pos: self.horizontal_pos, 
                depth: self.depth + count,
                aim: self.aim
            }  
        }
        else if direction == "forward" {  
            SubmarinePosition { 
                horizontal_pos: self.horizontal_pos + count, 
                depth: self.depth,
                aim: self.aim
            }  
        }
        else {
            println!("Unknown direction: {}.", direction);
            SubmarinePosition { 
                horizontal_pos: self.horizontal_pos, 
                depth: self.depth,
                aim: self.aim
            } 
        }
    }
    fn add_part_two(&self, direction: &str, count: i32) -> SubmarinePosition {
        
        if direction == "up" {  
            SubmarinePosition { 
                horizontal_pos: self.horizontal_pos, 
                depth: self.depth,
                aim: self.aim - count
            }  
        }
        else if direction == "down" {  
            SubmarinePosition { 
                horizontal_pos: self.horizontal_pos, 
                depth: self.depth,
                aim: self.aim + count
            }  
        }
        else if direction == "forward" {  
            SubmarinePosition { 
                horizontal_pos: self.horizontal_pos + count, 
                depth: self.depth + self.aim * count,
                aim: self.aim
            }  
        }
        else {
            println!("Unknown direction: {}.", direction);
            SubmarinePosition { 
                horizontal_pos: self.horizontal_pos, 
                depth: self.depth,
                aim: self.aim
            } 
        }
    }
    fn mul(&self) -> i32 {
        self.depth * self.horizontal_pos
    }
  }

fn main() {
    let contents = aoc::aoc::read_input(true);
    
    let (final_position_one, final_position_two) = contents.lines().map(|x| x.split(' ')).fold((SubmarinePosition::new(), SubmarinePosition::new()), |acc, x| { 
        let direction = {
            let mut x = x.clone();
            x.next().unwrap()
        };
        let count = i32::from_str(x.last().unwrap()).unwrap();

        (acc.0.add_part_one(direction, count), acc.1.add_part_two(direction, count))
    });

    println!("Result of part one = {}.", final_position_one.mul());
    println!("Result of part two = {}.", final_position_two.mul());
    
    ()
}
