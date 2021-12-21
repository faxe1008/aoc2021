use itertools::Itertools;

const SEGMENTS_ZERO: &str = "abcefg";
const SEGMENTS_ONE: &str = "cf";
const SEGMENTS_TWO: &str = "acdeg";
const SEGMENTS_THREE: &str = "acdfg";
const SEGMENTS_FOUR: &str = "bcdf";
const SEGMENTS_FIVE: &str = "abdfg";
const SEGMENTS_SIX: &str = "abdefg";
const SEGMENTS_SEVEN: &str = "acf";
const SEGMENTS_EIGHT: &str = "abcdefg";
const SEGMENTS_NINE: &str = "abcdfg";

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn sort_characters(s: &str) -> String {
    s.chars().sorted().collect()
}

#[derive(Default)]
struct ReportEntry {
    captured: Vec<String>,
    final_signal: Vec<String>,
    signal_mapping: HashMap<char, char>,
}

impl ReportEntry {
    fn new(line: &str) -> Result<Self, &'static str> {
        let report_components: Vec<&str> = line.trim().split('|').collect();
        if report_components.len() != 2 {
            return Err("Badly formated report line");
        }
        let captured = report_components[0]
            .split_whitespace()
            .map(|y| sort_characters(y))
            .collect();

        let final_signal = report_components[1]
            .split_whitespace()
            .map(|y| sort_characters(y))
            .collect();

        Ok(ReportEntry {
            captured,
            final_signal,
            signal_mapping: HashMap::new(),
        })
    }

    fn show(&self) {
        println!("{:?} => {:?}", self.captured, self.final_signal);
    }

    fn find_signal_with_length(&self, signals: &Vec<String>, len: usize) -> Vec<String> {
        signals
            .iter()
            .filter(|y| y.len() == len)
            .map(|y| y.clone())
            .collect()
    }

    fn signal_diff(&self, s1: &str, s2: &str) -> String {
        sort_characters(&s1.chars().filter(|&y| !s2.contains(y)).collect::<String>())
    }

    fn add_mapping(&mut self, src: char, dest: char) {
        assert!(!self.signal_mapping.contains_key(&src));
        self.signal_mapping.insert(src, dest);
    }

    fn solve(&mut self) {
        // get the codes for easy identifiable digits 1,4,7,8
        /*let one_segment = self
            .find_signal_with_length(&self.captured, 2)
            .get(0)
            .unwrap();
        let four_segment = self
            .find_signal_with_length(&self.captured, 4)
            .get(0)
            .unwrap();
        let seven_segment = self
            .find_signal_with_length(&self.captured, 3)
            .get(0)
            .unwrap();
        let eight_segment = self
            .find_signal_with_length(&self.captured, 7)
            .get(0)
            .unwrap();

        // from the difference between 7 and 1 we can get the top element 'a'
        let seven_to_one_diff = self.signal_diff(seven_segment, one_segment);
        assert!(seven_to_one_diff.len() == 1);
        self.add_mapping(seven_to_one_diff.chars().nth(0).unwrap(), 'a');*/
    }

    fn get_number_occs_in_final_signal(&self) -> usize {
        let mut count = 0;
        count += self.find_signal_with_length(&self.final_signal, 2).len();
        count += self.find_signal_with_length(&self.final_signal, 4).len();
        count += self.find_signal_with_length(&self.final_signal, 3).len();
        count += self.find_signal_with_length(&self.final_signal, 7).len();
        count
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Provide the depth input text file!");
    }

    let file = File::open(&args[1]).expect("Error opening file");
    let reader = BufReader::new(file);
    let mut reports = reader
        .lines()
        .map(|y| ReportEntry::new(&y.unwrap()).unwrap())
        .collect::<Vec<ReportEntry>>();

    let mut total_counter: usize = 0;
    for report in reports {
        println!(
            "{:?}: {}",
            report.final_signal,
            report.get_number_occs_in_final_signal()
        );
        total_counter += report.get_number_occs_in_final_signal();
    }
    println!("Total occurences of 1,4,7,8: {}", total_counter);
}
