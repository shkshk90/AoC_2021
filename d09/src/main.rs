extern crate aoc;



mod day9 {
    #[derive(Debug, Clone, Copy)]
    pub struct Point {
        val: u8,
        i: usize,
        j:usize, 
    }

    impl Point {
        pub fn risk_level(&self) -> u8 {
            self.val + 1
        }
    }

    pub fn low_points(grid: &Vec<Vec<u8>>) -> Vec<Point> {
        let mut result = Vec::new();
        
        let i_max = grid.len();
        let j_max = grid[0].len();

        let ie = i_max - 1;
        let je = j_max - 1;
        
        {
            let coordinates: [((usize, usize), (usize, usize), (usize, usize)); 4] = [
                ((0, 0), (0, 1), (1, 0)),
                ((0, je), (0, je - 1), (1, je)),
                ((ie, 0), (ie, 1), (ie - 1, 0)),
                ((ie, je), (ie, je - 1), (ie - 1, je)),
                ];
            
            for (c0, c1, c2) in coordinates.iter() {
                if grid[c0.0][c0.1] < grid[c1.0][c1.1] && grid[c0.0][c0.1] < grid[c2.0][c2.1] {
                    result.push(Point{
                        val: grid[c0.0][c0.1],
                        i: c0.0,
                        j: c0.1,
                    });
                }
            }
        } // Corners
        {
            for j in 1..je {
                if grid[0][j] < grid[0][j - 1] && 
                    grid[0][j] < grid[0][j + 1] &&
                    grid[0][j] < grid[1][j] {
                    result.push(Point{
                        val: grid[0][j],
                        i: 0,
                        j: j,
                    });
                }
                if grid[ie][j] < grid[ie][j - 1] && 
                    grid[ie][j] < grid[ie][j + 1] &&
                    grid[ie][j] < grid[ie - 1][j] {
                    result.push(Point{
                        val: grid[ie][j],
                        i: ie,
                        j: j,
                    });
                } 
            }
            for i in 1..ie {
                if grid[i][0] < grid[i - 1][0] && 
                grid[i][0] < grid[i + 1][0] && 
                grid[i][0] < grid[i][1] {
                    result.push(Point{
                        val: grid[i][0],
                        i: i,
                        j: 0,
                    });
                }
                if grid[i][je] < grid[i - 1][je] && 
                grid[i][je] < grid[i + 1][je] && 
                grid[i][je] < grid[i][je - 1] {
                    result.push(Point{
                        val: grid[i][je],
                        i: i,
                        j: je,
                    });
                }
            }
        } // Edges

        for i in 1..ie {
            for j in 1..je {
                if grid[i][j] < grid[i - 1][j] &&
                grid[i][j] < grid[i + 1][j] && 
                grid[i][j] < grid[i][j - 1] && 
                grid[i][j] < grid[i][j + 1] {
                    result.push(Point{
                        val: grid[i][j],
                        i: i,
                        j: j,
                    });
                }
            }
        }

        result
    }

    fn find_basin_rec(grid: &mut Vec<Vec<(u8, bool)>>, i: usize, j: usize, result: &mut Vec<Point>) -> () {
        if i == usize::MAX || i == grid.len()    { return }
        if j == usize::MAX || j == grid[0].len() { return }

        if grid[i][j].0 == 9 || grid[i][j].1 { return }

        result.push(Point{val: grid[i][j].0, i: i, j: j});
        grid[i][j].1 = true;

        let new_i_b = i.wrapping_sub(1);
        let new_j_b = j.wrapping_sub(1);
        let new_i_f = i + 1;
        let new_j_f = j + 1;

        find_basin_rec(grid, new_i_b, j, result);
        find_basin_rec(grid, new_i_f, j, result);
        find_basin_rec(grid, i, new_j_b, result);
        find_basin_rec(grid, i, new_j_f, result);
    }
    fn find_basin(grid: &Vec<Vec<u8>>, low_point: &Point) -> Vec<Point> {
        let mut result = Vec::new();
        let mut signed_grid = grid.iter()
            .map(|v| 
                v.iter()
                .map(|n| (*n, false))
                .collect::<Vec<(u8, bool)>>() )
            .collect::<Vec<Vec<(u8, bool)>>>();

        find_basin_rec(&mut signed_grid, low_point.i, low_point.j, &mut result);

        result
    }

    pub fn find_basins(grid: &Vec<Vec<u8>>, low_point_vec: &Vec<Point>) -> Vec<Vec<Point>> {
        low_point_vec.iter().map(|lp| find_basin(grid, lp)).collect::<Vec<Vec<Point>>>()
    }
}


fn main() {
    let contents = aoc::aoc::read_input(true);
    let _inputs = contents.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8 ).collect::<Vec<u8>>() ).collect::<Vec<Vec<u8>>>();
    
    let low_points = day9::low_points(&_inputs);
    let result: u32 = low_points.iter().map(|p| p.risk_level() as u32).sum();

    println!("Part one result = {}.", result);

    let result_2 = day9::find_basins(&_inputs, &low_points);
    let sizes_with_indices = result_2.iter().enumerate().map(|(i, v)| (i, v.len())).collect::<Vec<(usize, usize)>>();
    let sizes = {
        
        let s1 = sizes_with_indices.iter().fold((0, 0), |acc, (i, s)| if *s > acc.0 { (*s, *i) } else { acc });
        let s2 = sizes_with_indices.iter().filter(|(i, _)| *i != s1.1).fold((0, 0), |acc, (i, s)| if *s > acc.0 { (*s, *i) } else { acc });
        let s3 = sizes_with_indices.iter().filter(|(i, _)| *i != s1.1 && *i != s2.1).fold((0, 0), |acc, (i, s)| if *s > acc.0 { (*s, *i) } else { acc });

        (s1.0, s2.0, s3.0)
    };
    //println!("result 2 = {:?}", result_2);
    println!("Max three sizes = {} {} {}.", sizes.0, sizes.1, sizes.2);
    println!("Result = {}.", sizes.0 * sizes.1 * sizes.2);

    ()
}

