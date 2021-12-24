extern crate aoc;

mod day13 {
    pub fn make_grid(v: &Vec<(usize, usize)>, max_x: usize, max_y: usize) -> Vec<Vec<bool>> {
        let mut result = vec![vec![false; max_x + 1]; max_y + 1];

        v.iter().for_each(|(i, j)| {
            result[*j][*i] = true;
        });

        result
    }

    pub fn print_grid(v: &Vec<Vec<bool>>) -> () {
        v.iter().for_each(|v| {
                v.iter().for_each(|b| {
                    print!("{}", if *b { '#' } else { '.' });
                });
                println!("");
            }
        );
    }

    fn fold_x(grid: &Vec<Vec<bool>>, at_x: usize) -> Vec<Vec<bool>> {
        let fst_half = grid.iter()
            .map(|v| v.iter().take(at_x).map(|&b| b ).collect::<Vec<bool>>() );
        let sec_half = grid.iter()
            .map(|v| v.iter().skip(at_x).rev().map(|&b| b ).collect::<Vec<bool>>() );
        

        fst_half.zip(sec_half)
            .map(|(va, vb)| 
                va.iter()
                    .zip(vb.iter())
                    .map(|(a, b)| *a || *b)
                    .collect::<Vec<bool>>())
            .collect::<Vec<Vec<bool>>>()
    }
    fn fold_y(grid: &Vec<Vec<bool>>, at_y: usize) -> Vec<Vec<bool>> {
        let fst_half = grid.iter().take(at_y);
        let sec_half = grid.iter().skip(at_y).rev();
        
        fst_half.zip(sec_half)
            .map(|(va, vb)| 
                va.iter()
                    .zip(vb.iter())
                    .map(|(a, b)| *a || *b)
                    .collect::<Vec<bool>>())
            .collect::<Vec<Vec<bool>>>()
    }

    pub fn fold(grid: &Vec<Vec<bool>>, at: &(char, usize)) -> Vec<Vec<bool>> {
        match at.0 {
            'x'  =>  fold_x(grid, at.1),
            'y'  =>  fold_y(grid, at.1),
             _   =>  Vec::new()
        }
    }
}

fn main() {
    let contents = if cfg!(feature = "example") { aoc::aoc::read_example(true) } else { aoc::aoc::read_input(true) };
    let _inputs = contents.lines();

    let _origami = _inputs
        .clone()
        .take_while(|line| line.is_empty() == false)
        .map(|s| s.split(',').collect::<Vec<&str>>() )
        .map(|v| (v[0], v[1]) )
        .map(|(s0, s1)| (s0.parse::<usize>().unwrap(), s1.parse::<usize>().unwrap()))
        .collect::<Vec<(usize, usize)>>();
    let _steps = _inputs
        .clone()
        .skip_while(|line| line.is_empty() == false)
        .skip(1)
        .map(|s|  s.split(' ').collect::<Vec<&str>>())
        .map(|vs| vs[2].split('=').collect::<Vec<&str>>())
        .map(|vs| (vs[0].chars().next().unwrap(), vs[1].parse::<usize>().unwrap()))
        .collect::<Vec<(char, usize)>>();
    
   let (max_x, max_y) = _origami.iter()
        .fold((usize::MIN, usize::MIN), 
            |(max_x_acc, max_y_acc), (x, y)| 
            (
                if *x > max_x_acc { *x } else { max_x_acc },
                if *y > max_y_acc { *y } else { max_y_acc }
            )
        );
    
    let grid = day13::make_grid(&_origami, max_x, max_y);
    let folded_grid = _steps.iter().take(1).fold(grid.clone(), |acc, step| day13::fold(&acc, step));
    
    //day13::print_grid(&folded_grid);
    
    let part_one = folded_grid.iter().fold(0, |acc, vec_dot| {
        acc + vec_dot.iter().fold(0, |acc, dot| if *dot { acc + 1 } else { acc })
    });
    let part_two = 0;


    println!("Part One = {}.", part_one);
    println!("Part Two = {}.", part_two);

    let folded_grid_ii = _steps.iter().fold(grid, |acc, step| day13::fold(&acc, step));
    
    day13::print_grid(&folded_grid_ii);

    ()
}
