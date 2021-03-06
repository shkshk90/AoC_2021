pub mod aoc {
    use std::fs;
    use std::env;

    fn read_file(verbose: bool, file_name: &str) -> String {
        let current_path_res = env::current_dir();
        let current_path_opt = match current_path_res {
            Ok(dir_path) => {
                if verbose {
                    println!("The current directory is: {}", dir_path.display()); 
                }
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
            current_path.push(file_name);
    
            current_path
        };
        let _path_to_print = current_path.clone();
        let contents = fs::read_to_string(current_path)
            .expect("Failed to open file {}.");
        
        if verbose {
            println!("Contents of file {} are retrieved successfully.", _path_to_print.display());
        }
        
        contents
    }

    pub fn read_input(verbose: bool) -> String {
        read_file(verbose, "input.txt") 
    }
    pub fn read_example(verbose: bool) -> String {
        read_file(verbose, "example.txt") 
    }
 }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
