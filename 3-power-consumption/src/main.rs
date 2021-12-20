#![feature(vec_retain_mut)]
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

fn has_bit_set(val: u32, bit: u32) -> bool {
    ((val & (1 << bit)) >> bit) == 1
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Provide the report input text file!");
    }

    let file = File::open(&args[1]).expect("Error opening file");
    let reader = BufReader::new(file);
    let mut o2_list: Vec<u32> = reader
        .lines()
        .map(|x| u32::from_str_radix(&x.unwrap(), 2).expect("Error converting"))
        .collect();

    let mut co2_list = o2_list.clone();

    for i in 0..12 {
        let (zero_occ, one_occ) = count_occurences(&o2_list, i);
        if one_occ > zero_occ {
            if o2_list.len() > 1 {
                o2_list.retain_mut(|x| has_bit_set(*x, i));
            }
            if co2_list.len() > 1 {
                co2_list.retain_mut(|x| !has_bit_set(*x, i))
            }
        } else if zero_occ > one_occ {
            if o2_list.len() > 1 {
                o2_list.retain_mut(|x| !has_bit_set(*x, i));
            }
            if co2_list.len() > 1 {
                co2_list.retain_mut(|x| has_bit_set(*x, i));
            }
        } else {
            if o2_list.len() > 1 {
                o2_list.retain_mut(|x| has_bit_set(*x, i));
            }
            if co2_list.len() > 1 {
                co2_list.retain_mut(|x| !has_bit_set(*x, i));
            }
        }
    }
    println!("O2 Codes: {:?}", o2_list);
    println!("CO2 Codes: {:?}", co2_list);
}
