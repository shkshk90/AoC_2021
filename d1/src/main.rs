use std::env;
use std::fs;

enum Step {
    NoChange,
    Increasing,
    Decreasing
}

fn main() {
    let current_path_res = env::current_dir();
    let current_path_opt =
    match current_path_res {
        Ok(dir_path) => {
            println!("The current directory is: {}", dir_path.display()); 
            Some(dir_path.clone())
        },
        Err(e) => {
            println!("Failed to get current dir path: {}", e); 
            None
        },
    };
    if current_path_opt.is_none() { () }
    let current_path =  {
        let mut current_path = current_path_opt.unwrap();
        
        current_path.push("data");
        current_path.push("input.txt");

        current_path
    };
    let contents = fs::read_to_string(current_path)
        .expect("Failed to open file {}.");
    {
        let mut numbers = contents.lines()
                            .map( |x| x.parse::<u32>().unwrap());
        let mut prev_num = numbers.next().unwrap();
        let result = numbers.map( |x| {
            let res = if x > prev_num { Step::Increasing } else { Step::Decreasing }; 
            prev_num = x; 
            res} 
        ).fold(0, |acc, x| acc + if matches!(x, Step::Increasing) { 1 } else { 0 } );

        println!("Result A = {}.", result);
    }

    {
        let numbers_a = contents.lines()
                        .map( |x| x.parse::<u32>().unwrap());
        let numbers_b = {
            let mut numbers = contents.lines()
                        .map( |x| x.parse::<u32>().unwrap());
            numbers.next();
            numbers
        };
        let numbers_c = {
            let mut numbers = contents.lines()
                        .map( |x| x.parse::<u32>().unwrap());
            numbers.next();
            numbers.next();
            numbers
        };

        let mut numbers = numbers_c.zip(numbers_b).zip(numbers_a).map( |x| {
            let a = x.0.0;
            let b = x.0.1;
            let c = x.1;

            a + b + c
        } );
        let mut prev_num = numbers.next().unwrap();
        let result = numbers.map( |x| {
            let res = if x > prev_num { Step::Increasing } else if x == prev_num { Step::NoChange } else { Step::Decreasing }; 
            prev_num = x; 
            res} 
        ).fold(0, |acc, x| acc + if matches!(x, Step::Increasing) { 1 } else { 0 } );

        println!("Result B = {}.", result);

    }
}
