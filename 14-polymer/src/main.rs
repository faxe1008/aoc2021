use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn polymerize(polymer: &str, rules: &HashMap<&str, &str>) -> String {
    let mut new_polymer = String::new();
    for i in 0..polymer.len() - 1 {
        if let Some(new_sign) = rules.get(&polymer[i..=i + 1]) {
            new_polymer.push_str(&polymer[i..i + 1]);
            new_polymer.push_str(&new_sign);
        }
    }
    new_polymer.push(polymer.chars().last().unwrap());
    new_polymer
}

fn get_occurence_count(text: &str) -> HashMap<char, usize> {
    let mut letter_counts: HashMap<char, usize> = HashMap::new();
    let char_vec: Vec<char> = text.to_lowercase().chars().collect();
    for c in char_vec {
        *letter_counts.entry(c).or_insert(0) += 1;
    }
    letter_counts
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Provide the polymer input text file!");
    }

    let file = File::open(&args[1]).expect("Error opening file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|y| y.unwrap()).collect();

    assert!(lines.len() >= 2);
    let mut polymer = lines.iter().nth(0).unwrap().trim().to_owned();

    let mut rules: HashMap<&str, &str> = HashMap::new();
    for line in lines.iter().skip(2) {
        let components: Vec<&str> = line.split(" -> ").map(|y| y.trim()).collect();
        assert!(components.len() == 2);
        rules.insert(components[0], components[1]);
    }

    println!("Polymer: {}", polymer);
    println!("Rules: {:?}", rules);
    println!("");
    println!("");

    const POLYMERIZATION_COUNT: usize = 10;
    for i in 1..=POLYMERIZATION_COUNT {
        let new_polymer = polymerize(&polymer, &rules);
        polymer = new_polymer;
    }
    println!(
        "{} - polymer({}) is {}",
        POLYMERIZATION_COUNT,
        polymer.len(),
        polymer
    );

    let occurence_map = get_occurence_count(&polymer);
    let min_occ = occurence_map.values().min().unwrap();
    let max_occ = occurence_map.values().max().unwrap();

    println!(
        "Most common: {}, Least common: {} == {}",
        min_occ,
        max_occ,
        max_occ - min_occ
    );
}
