use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeBounds;

#[derive(Debug, Default)]
struct SyntaxChecker {
    symbol_stack: Vec<char>,
}

impl SyntaxChecker {
    fn find_key_for_value<'a>(&self, map: &'a HashMap<char, char>, value: char) -> Option<char> {
        map.iter().find_map(|(key, &val)| {
            if val == value {
                Some(key.clone())
            } else {
                None
            }
        })
    }

    fn check(&mut self, text: &str) -> Vec<char> {
        let mut errors = Vec::new();
        let all_signs = text.trim().chars();

        let syntax_mapping: HashMap<char, char> =
            HashMap::from([('(', ')'), ('{', '}'), ('<', '>'), ('[', ']')]);

        for sign in all_signs {
            if syntax_mapping.contains_key(&sign) {
                self.symbol_stack.push(sign);
            } else {
                assert!(syntax_mapping
                    .values()
                    .collect::<Vec<&char>>()
                    .contains(&&sign));
                let expected_stack_sym = self.find_key_for_value(&syntax_mapping, sign).unwrap();
                let stack_top = self.symbol_stack.pop();

                if stack_top.is_some() && stack_top.unwrap() != expected_stack_sym {
                    // we have an error
                    let expected_current_symbol = syntax_mapping.get(&stack_top.unwrap()).unwrap();
                    println!("Expected {}, but found {}", expected_current_symbol, sign);
                    errors.push(sign);
                    break;
                }
            }
        }
        errors
    }

    fn get_error_score(&self, errors: &Vec<char>) -> u32 {
        errors
            .iter()
            .map(|x| match x {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => 0,
            })
            .sum()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Provide the depth input text file!");
    }

    let file = File::open(&args[1]).expect("Error opening file");
    let reader = BufReader::new(file);

    let mut total_score = 0;
    for line in reader.lines() {
        let line_text = line.unwrap();
        let mut syntax_checker = SyntaxChecker::default();
        let errors = syntax_checker.check(&line_text);
        if errors.is_empty() {
            continue;
        }
        total_score += syntax_checker.get_error_score(&errors);
    }
    println!("Total error score is: {}", total_score);
}
