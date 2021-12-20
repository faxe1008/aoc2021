use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Provide the depth input text file!");
    }

    let file = File::open(&args[1]).expect("Error opening file");
    let reader = BufReader::new(file);

    let mut depth_increases = 0;
    let mut last_depth: Option<u32> = None;
    for line in reader.lines() {
        let depth: u32 = line.unwrap().parse().expect("Error converting to int");
        if let Some(l_depth) = last_depth {
            if depth > l_depth {
                depth_increases += 1;
            }
        }
        last_depth = Some(depth);
    }
    println!("Got {} depth increases", depth_increases);
}
