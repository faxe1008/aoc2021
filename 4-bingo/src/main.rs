use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default)]
struct BingoCell {
    number: u32,
    checked: bool,
}

struct BingoBoard {
    cells: Vec<BingoCell>,
}

impl BingoBoard {
    fn new(lines: &[String]) -> BingoBoard {
        assert!(lines.len() == 5);
        let mut cells: Vec<BingoCell> = Vec::new();
        for line in lines {
            let numbers: Vec<u32> = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            for number in numbers {
                cells.push(BingoCell {
                    number,
                    checked: false,
                });
            }
        }
        BingoBoard { cells }
    }

    fn ugly_print(&self) {
        for i in 0..self.cells.len() {
            if self.cells[i].checked {
                print!("\x1b[93m{:02}\x1b[0m ", self.cells[i].number);
            } else {
                print!("{:02} ", self.cells[i].number);
            }
            if (i as i32 - 4) % 5 == 0 {
                print!("\n")
            }
        }
    }

    fn mark_number(&mut self, number: u32) {
        for cell in self.cells.iter_mut() {
            if cell.number == number {
                cell.checked = true;
            }
        }
    }

    fn has_bingo(&self) -> bool {
        //check rows
        for row_index in 0..5 {
            let cells_checked = self.cells[row_index * 5..(row_index + 1) * 5]
                .iter()
                .filter(|x| x.checked)
                .collect::<Vec<&BingoCell>>()
                .len();
            if cells_checked == 5 {
                return true;
            }
        }

        //check columns
        'outer: for column_index in 0..5 {
            for pos in (column_index..self.cells.len()).step_by(5) {
                if !self.cells[pos].checked {
                    continue 'outer;
                }
            }
            return true;
        }
        false
    }

    fn sum(&self) -> u32 {
        self.cells
            .iter()
            .filter(|y| !y.checked)
            .map(|x| x.number)
            .sum()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Provide the bingo input text file!");
    }

    let file = File::open(&args[1]).expect("Error opening file");
    let reader = BufReader::new(file);
    let input_lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

    let called_out_numbers: Vec<u32> = input_lines[0]
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut boards: Vec<BingoBoard> = Vec::new();
    for i in (1..input_lines.len()).step_by(6) {
        if i + 6 > input_lines.len() {
            break;
        }
        let board = BingoBoard::new(&input_lines[i + 1..i + 6]);
        boards.push(board);
    }

    'number_loop: for bingo_number in called_out_numbers {
        for board in &mut boards {
            board.mark_number(bingo_number);
            if board.has_bingo() {
                println!("Board has bingo!!!");
                board.ugly_print();
                println!("Board sum is: {}", board.sum());
                println!("Current number is: {}", bingo_number);
                break 'number_loop;
            }
        }
    }
}
