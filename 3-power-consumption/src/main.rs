use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_occurences(report: &Vec<u32>, bit: u32) -> (u32, u32) {
    let number_of_ones = report.into_iter().map(|x| (x & (1 << bit)) >> bit).sum();
    (report.len() as u32 - number_of_ones, number_of_ones)
}

fn set_bit(val: u32, bit: u32, active: bool) -> u32 {
    if active {
        val | (1 << bit)
    } else {
        val & !(1 << bit)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Provide the report input text file!");
    }

    let file = File::open(&args[1]).expect("Error opening file");
    let reader = BufReader::new(file);
    let report_codes: Vec<u32> = reader
        .lines()
        .map(|x| u32::from_str_radix(&x.unwrap(), 2).expect("Error converting"))
        .collect();

    let mut gamma_rate: u32 = 0;
    let mut epsilon_rate: u32 = 0;
    for i in 0..12 {
        let (zero_occ, one_occ) = count_occurences(&report_codes, i);
        println!("Bit Nr. {} ========== 0: {}, 1: {}", i, zero_occ, one_occ);
        if one_occ > zero_occ {
            gamma_rate = set_bit(gamma_rate, i, true);
            epsilon_rate = set_bit(epsilon_rate, i, false);
        } else {
            gamma_rate = set_bit(gamma_rate, i, false);
            epsilon_rate = set_bit(epsilon_rate, i, true);
        }
    }

    println!("Gamma,Epsilon: {}, {}", gamma_rate, epsilon_rate);
}
