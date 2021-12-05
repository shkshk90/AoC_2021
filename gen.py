import subprocess
from os import makedirs, utime
from os.path import join

S_MAIN = '''
extern crate aoc;

use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone)]
struct Test {
    x: u32
}

impl Test {
    fn new() -> Self {
        Self {
            x: 0_u32
        }
    }
}


fn main() {
    let contents = aoc::aoc::read_input(true);
    let _inputs = contents.lines();
    
    ()
}

'''

for i in range(5, 25):
  
  s_dir_name = 'd%02i' % i
  s_data_dir = join(s_dir_name, 'data')
  s_input_txt = join(s_data_dir, 'input.txt')
  s_example_txt = join(s_data_dir, 'example.txt')
  s_example_txt = join(s_data_dir, 'example.txt')
  s_cargo_toml= join(s_dir_name, 'cargo.toml')
  s_src = join(s_dir_name, 'src', 'main.rs')
  
  if False:
    print(s_dir_name)

    subprocess.run(['cargo', 'new', '--bin', '--vcs', 'none', s_dir_name])
    makedirs(s_data_dir)
    
    with open(s_input_txt, 'a'):
      utime(s_input_txt, None)
    with open(s_example_txt, 'a'):
      utime(s_example_txt, None)
  
  if False:
    ls_lines = None
    with open(s_cargo_toml, 'r') as f:
      ls_lines = f.readlines()
    with open(s_cargo_toml, 'w') as f:
      for line in ls_lines:
        f.write(line)
        if line.find("dependencies") != -1:
          f.write(r'aoc = { path = "../aoc" }')
  
  if True:
    with open(s_src, 'w') as f:
      f.write(S_MAIN)



