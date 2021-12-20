use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const SLIDING_WINDOW_SIZE: usize = 3;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Provide the depth input text file!");
    }

    let file = File::open(&args[1]).expect("Error opening file");
    let reader = BufReader::new(file);
    let depths = reader
        .lines()
        .map(|x| x.unwrap().parse().expect("Error converting to int"))
        .collect::<Vec<u32>>();

    let mut depth_increases = 0;
    let mut last_depth: Option<u32> = None;
    for index in 0..depths.len() {
        if index + SLIDING_WINDOW_SIZE >= depths.len() {
            continue;
        }
        let depth_sum: u32 = depths[index..index + SLIDING_WINDOW_SIZE].iter().sum();
        if let Some(l_depth) = last_depth {
            if depth_sum > l_depth {
                depth_increases += 1;
            }
        }
        last_depth = Some(depth_sum);
    }
    println!("Got {} depth increases", depth_increases);
}
