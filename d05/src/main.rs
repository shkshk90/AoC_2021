
extern crate aoc;

use std::str::FromStr;
use std::cmp;

#[derive(Clone, Debug)]
struct Point {
    x: usize,
    y: usize
}

impl Point {
    fn _new() -> Self {
        Self {
            x: usize::MAX,
            y: usize::MAX
        }
    }
    fn new_from_str(point: &str) -> Self {
        let res = point.split(',').map(|num_str| usize::from_str(num_str).unwrap()).collect::<Vec<usize>>();
        Self { x: res[0], y: res[1] }
    }
    fn _increment(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
    fn increment_towards(&self, p: &Point) -> Option<Point> {
        if self.x == p.x || self.y == p.y {
            None 
        } else {
            Some(Point {
                x: if p.x > self.x { self.x + 1 } else {self.x - 1},
                y: if p.y > self.y { self.y + 1 } else {self.y - 1},
            })
        }
    }
}

#[derive(Clone, Debug)]
struct Line {
    start: Point,
    end: Point
}

impl Line {
    fn from_points(p1: Point, p2: Point) -> Self {
        Self {
            start: Point {
                // x: if p1.x < p2.x { p1.x } else { p2.x },
                // y: if p1.y < p2.y { p1.y } else { p2.y },
                x: p1.x,
                y: p1.y
            },
            end: Point {
                // x: if p1.x > p2.x { p1.x } else { p2.x }, 
                // y: if p1.y > p2.y { p1.y } else { p2.y },
                x: p2.x,
                y: p2.y
            }
        }
    }
    fn _point_is_covered(&self, p: &Point) -> bool {
        (p.x <= self.end.x) && (p.y <= self.end.y) 
    }
}

#[derive(Clone)]
struct Diagram {
    lines: Vec<Line>,
    grid: Vec<u8>,
    grid2: Vec<u8>,
    cols: usize,
    rows: usize
}

impl Diagram {
    fn from_inputs(inputs: Vec<(&str, &str)>) -> Self {
        let points = inputs.iter()
            .map(|(src_str, dst_str)| (Point::new_from_str(src_str), Point::new_from_str(dst_str)))
            .map(|(p1, p2)| Line::from_points(p1, p2))
            .collect::<Vec<Line>>();
        let (rows_, cols_) = points.iter()
            .fold((0_usize, 0_usize), |(row_acc, col_acc), line| (cmp::max(cmp::max(line.end.x, line.start.x), row_acc), cmp::max(cmp::max(line.end.y, line.start.y), col_acc)) );
        
        let rows = rows_ + 1;
        let cols = cols_ + 1;

        let empty_diagram = Diagram {
            lines: points,
            grid: vec![0u8; rows * cols ],
            grid2: vec![0u8; rows * cols ],
            cols: cols,
            rows: rows,
        };

        Diagram::parse_grid(&empty_diagram)
    }

    fn parse_grid(empty_diagram: &Diagram) -> Diagram {
        let mut result = empty_diagram.clone();
        empty_diagram.lines.iter().for_each(|line| {
            println!("line is {:?}", line);
            if line.start.x == line.end.x {
                let x = line.start.x;
                let (start, end) = if line.start.y < line.end.y { (line.start.y, line.end.y) } else { (line.end.y, line.start.y) };
                for i in start..=end {
                    result.grid[i * empty_diagram.cols + x] += 1;
                    result.grid2[i * empty_diagram.cols + x] += 1;
                }
            }
            else if line.start.y == line.end.y {
                let base_y = line.start.y * empty_diagram.cols;
                let (start, end) = if line.start.x < line.end.x { (line.start.x, line.end.x) } else { (line.end.x, line.start.x) };
                for i in start..=end {
                    result.grid[base_y + i] += 1;
                    result.grid2[base_y + i] += 1;
                }
            } else {
                let mut point = line.start.clone();
                loop {
                    let index = point.x + point.y * empty_diagram.cols;
                    result.grid2[index] += 1;

                    let point_opt = point.increment_towards(&line.end);
                    match point_opt {
                        None => break,
                        Some(pnt) => point = pnt,
                    };
                }
            }
        });

        result
    }

    fn _print_grid(&self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                let val = self.grid[i * self.cols + j];
                print!("{}", if val == 0 { format!("{}", '.') } else { format!("{}", val) });
            }
            print!("\n");
        }
        print!("\n");
    }
    fn _print_grid2(&self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                let val = self.grid2[i * self.cols + j];
                print!("{}", if val == 0 { format!("{}", '.') } else { format!("{}", val) });
            }
            print!("\n");
        }
        print!("\n");
    }

    fn safer_areas_count(&self) -> usize {
        self.grid.iter().filter(|x| **x > 1).count()
    }
    fn safer_areas_count_with_diag(&self) -> usize {
        self.grid2.iter().filter(|x| **x > 1).count()
    }
}


fn main() {
    let contents = aoc::aoc::read_input(true);
    let _inputs = contents.lines();

    let inputs_as_pairs = _inputs
        .map(|line| line
            .split_whitespace()
            .filter(|str| str.contains(","))
            .collect::<Vec<&str>>() )
        .map(|vec_str| (vec_str[0], vec_str[1]))
        .collect::<Vec<(&str, &str)>>();

    let diagram = Diagram::from_inputs(inputs_as_pairs);
    
    println!("grid is {} x {}:", diagram.rows, diagram.cols);
    println!("Result 1 is {}.", diagram.safer_areas_count());
    println!("Result 2 is {}.", diagram.safer_areas_count_with_diag());
    
    ()
}

