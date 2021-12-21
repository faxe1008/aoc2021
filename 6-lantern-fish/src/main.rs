use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Provide the depth input text file!");
    }

    let text = fs::read_to_string(&args[1]).expect("Error reading file");
    let mut fish_swarm: Vec<u8> = text
        .trim()
        .split(',')
        .map(|x| x.parse().expect("Error converting to int"))
        .collect();

    // set up the table of how many fish have which timer
    let mut fish_map: HashMap<u8, usize> = HashMap::new();
    for i in 0..=8 {
        fish_map.insert(i, fish_swarm.iter().filter(|&&y| y == i).count());
    }

    let generations: usize = 256;
    for i in 1..=generations {
        let mut next_fish_map: HashMap<u8, usize> = HashMap::new();

        *next_fish_map.entry(6).or_insert(0) += fish_map.get(&0).unwrap_or(&0); // reset
        *next_fish_map.entry(8).or_insert(0) += fish_map.get(&0).unwrap_or(&0); // new borns

        for x in 0..=8 {
            *next_fish_map.entry(x).or_insert(0) += fish_map.get(&(x + 1)).unwrap_or(&0);
            //Next gens
        }

        fish_map = next_fish_map.clone();
        println!("Day {}: {:#?}", i, fish_map);
    }
    let fish_count: usize = fish_map.into_iter().map(|(_timer, occ)| occ).sum();
    println!("There are {} fish", fish_count);
}
