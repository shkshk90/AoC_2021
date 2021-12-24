extern crate aoc;

mod day15 {
    use std::collections::HashMap;
    use std::collections::HashSet;

    fn to_fat_index(i: usize, j: usize) -> usize { 
        (i << 32) | j
    }
    fn from_fat_index(fi: usize) -> (usize, usize) { 
        (fi >> 32, fi & 0xFFFFFFFF)
    }
    pub fn dijkstra2(graph: &Vec<Vec<u8>>) -> Vec<Vec<(usize, usize)>> {
        let mut q: HashSet<usize>           = HashSet::with_capacity(graph.len() * graph[0].len());
        let mut dist: HashMap<usize, usize> = HashMap::with_capacity(graph.len() * graph[0].len());

        let mut prev: Vec<Vec<(usize, usize)>> = vec![vec![(0, 0); graph[0].len()]; graph.len()];
        let mut u = (0, 0);

        (0..graph.len()).for_each(|i| {
            (0..graph[0].len()).for_each(|j| {
                q.insert(to_fat_index(i, j));
            });
        });

        dist.insert(to_fat_index(0, 0), 0);

        while q.len() > 0 {
            if u.0 == graph.len() - 1 && u.1 == graph[0].len() - 1 { break }

            let ukey = to_fat_index(u.0, u.1);
            let dist_u = *(dist.get(&ukey).unwrap());

            for (i, j) in [ 
                (u.0, u.1 + 1), 
                (u.0 + 1, u.1),
                (u.0, u.1.overflowing_sub(1).0), 
                (u.0.overflowing_sub(1).0, u.1), 
            ] {
                let vkey =  to_fat_index(i, j);
                if q.contains(&vkey) == false       { continue }
                
                let diff = (graph[i][j]) as usize;
                let alt = dist_u + diff;

                let dist_v = dist.get_mut(&vkey); 
                match dist_v {
                    Some(val_ref) => {
                        if alt < *val_ref {
                            *val_ref = alt;
                            prev[i][j] = (u.0, u.1);
                        }
                    },
                    None => {
                        dist.insert(vkey, alt);
                        prev[i][j] = (u.0, u.1);
                    },
                };
            }

            q.remove(&ukey);
            dist.remove(&ukey);

            let (_, i, j) = {
                let mut min = usize::MAX;
                let mut ii = usize::MAX;
                let mut jj = usize::MAX;

                dist.iter()
                    .filter(|(k, _)|  q.contains(k))
                    .for_each(|(&k, &v)| {
                    let (i, j) = from_fat_index(k);

                    if v < min {
                        min = v; 
                        ii = i;
                        jj = j;
                    }
                });

                (min, ii, jj)
            };

            u = (i, j);
        }

        prev
    }
    pub fn dijkstra3(graph: &Vec<Vec<u8>>) -> Vec<Vec<(usize, usize)>> {
        let mut q = vec![vec![true; graph[0].len()]; graph.len()];
        let mut dist: HashMap<usize, usize> = HashMap::with_capacity(graph.len() * graph[0].len());

        let mut prev: Vec<Vec<(usize, usize)>> = vec![vec![(0, 0); graph[0].len()]; graph.len()];
        let mut u = (0, 0);

        dist.insert(to_fat_index(0, 0), 0);

        while q.len() > 0 {
            if u.0 == graph.len() - 1 && u.1 == graph[0].len() - 1 { break }

            let ukey = to_fat_index(u.0, u.1);
            let dist_u = *(dist.get(&ukey).unwrap());

            for (i, j) in [ 
                (u.0, u.1 + 1), 
                (u.0 + 1, u.1),
                (u.0, u.1.overflowing_sub(1).0), 
                (u.0.overflowing_sub(1).0, u.1), 
            ] {
                let vkey =  to_fat_index(i, j);
                
                if i >= graph.len()         { continue }
                if j >= graph[0].len()      { continue }
                if q[i][j] == false         { continue }
                
                let diff = (graph[i][j]) as usize;
                let alt = dist_u + diff;

                let dist_v = dist.get_mut(&vkey); 
                match dist_v {
                    Some(val_ref) => {
                        if alt < *val_ref {
                            *val_ref = alt;
                            prev[i][j] = (u.0, u.1);
                        }
                    },
                    None => {
                        dist.insert(vkey, alt);
                        prev[i][j] = (u.0, u.1);
                    },
                };
            }

            q[u.0][u.1] = false;
            dist.remove(&ukey);

            let (_, i, j) = {
                let mut min = usize::MAX;
                let mut ii = usize::MAX;
                let mut jj = usize::MAX;

                dist.iter()
                    .map(|(k, v)| (from_fat_index(*k), v))
                    .filter(|((i, j), _)|  q[*i][*j]  )
                    .for_each(|((i, j), &v)| 
                    if v < min {
                        min = v; 
                        ii = i;
                        jj = j;
                    }
                );

                (min, ii, jj)
            };

            u = (i, j);
        }

        prev
    }
    pub fn get_path(dv: &Vec<Vec<(usize, usize)>>) -> Vec<(usize, usize)> {
        let li = dv.len() - 1;
        let lj = dv[0].len() - 1;

        let mut result = Vec::new();

        let (mut i, mut j) = (li, lj);

        loop {
            result.push((i, j));

            if i == 0 && j == 0 { break }
            
            let p = dv[i][j];

            i = p.0;
            j = p.1;
        }

        result.into_iter().rev().collect()
    }
    pub fn calc_path(graph: &Vec<Vec<u8>>, path: &Vec<(usize, usize)>) -> usize {
        path.iter().skip(1).fold(0_usize, |acc, (i, j)| acc + (graph[*i][*j] as usize))
    }
    pub fn expand(graph: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        let li = graph.len();
        let lj = graph[0].len();
        let mut result = vec![vec![0; lj * 5]; li * 5];

        graph.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, c)| {
                (0..5).for_each(|ii| {
                    (0..5).for_each(|jj| {
                        
                        let to_add = (ii + jj) as u8;
                        let new_val = c + to_add;
                        let final_val = if new_val > 9 { new_val - 9 } else { new_val };

                        result[i + ii * li][j + jj * lj] = final_val;
                    })
                });
            })
        });

        result
    }
}

fn main() {
    let contents = if cfg!(feature = "example") { aoc::aoc::read_example(true) } else { aoc::aoc::read_input(true) };
    let _inputs = contents.lines();
    let _cavern = {
        let mut cav = _inputs.clone()
            .map(|line| line.chars().map(|c| (c as u8) - ('0' as u8)).collect::<Vec<u8>>()  )
            .collect::<Vec<Vec<u8>>>();
        cav[0][0] = 0;

        cav
    };
    let _cavern2 = _inputs.clone()
        .map(|line| line.chars().map(|c| (c as u8) - ('0' as u8)).collect::<Vec<u8>>()  )
        .collect::<Vec<Vec<u8>>>();

    

    let part_one = {
        let dijkstra = day15::dijkstra2(&_cavern);
        let path = day15::get_path(&dijkstra);

        day15::calc_path(&_cavern, &path)
    };
    
    let expanded = day15::expand(&_cavern2);
    let part_two = {
        let dijkstra = day15::dijkstra3(&expanded);
        let path = day15::get_path(&dijkstra);

        day15::calc_path(&expanded, &path)
    };

    println!("Part One = {}.", part_one);
    println!("Part Two = {}.", part_two);

    ()
}
