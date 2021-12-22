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
        let one_segment_candidates = self.find_signal_with_length(&self.captured, 2);
        let four_segment_candidates = self.find_signal_with_length(&self.captured, 4);
        let seven_segment_candidates = self.find_signal_with_length(&self.captured, 3);
        let eight_segment_candidates = self.find_signal_with_length(&self.captured, 7);

        let one_segment = one_segment_candidates.get(0).unwrap();
        let four_segment = four_segment_candidates.get(0).unwrap();
        let seven_segment = seven_segment_candidates.get(0).unwrap();
        let eight_segment = eight_segment_candidates.get(0).unwrap();

        // STARTING THE SOLVE

        // from the difference between 7 and 1 we can get the top element 'a'
        let seven_to_one_diff = self.signal_diff(seven_segment, one_segment);
        assert!(seven_to_one_diff.len() == 1);
        self.add_mapping(seven_to_one_diff.chars().nth(0).unwrap(), 'a');

        // The digits with 6 segments are 0, 6, and 9
        // We can identify them with the following statements:
        // 9 has ALL of 4 and ALL of 1's segments
        // 0 Doesn't have ALL of 4's segments, but does have ALL of 1's segments
        // 6 Doesn't have ALL of 4 and 1's segments

        let zero_six_nine_candidates = self.find_signal_with_length(&self.captured, 6);
        assert!(zero_six_nine_candidates.len() == 3);
        let mut nine_segment: Option<String> = None;
        let mut zero_segment: Option<String> = None;
        for candidate in zero_six_nine_candidates {
            if self.signal_diff(&candidate, &four_segment).len() == 0
                && self.signal_diff(&candidate, &one_segment).len() == 0
            {
                assert!(nine_segment.is_none());
                nine_segment = Some(candidate.clone());
            }

            if self.signal_diff(&candidate, &four_segment).len() != 0
                && self.signal_diff(&candidate, &one_segment).len() == 0
            {
                assert!(zero_segment.is_none());
                zero_segment = Some(candidate.clone());
            }
        }

        println!("{:?}", nine_segment);
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

    for report in reports.iter_mut() {
        report.solve();
    }
}
