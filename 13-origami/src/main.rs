use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const PAPER_WIDTH: usize = 1350;
const PAPER_HEIGHT: usize = 1350;

struct Paper {
    width: usize,
    height: usize,
    matrix: Vec<bool>,
}

impl Paper {
    fn new() -> Self {
        Paper {
            width: PAPER_WIDTH,
            height: PAPER_HEIGHT,
            matrix: vec![false; PAPER_WIDTH * PAPER_HEIGHT],
        }
    }

    fn get_by_coordinate_mut(&mut self, x: usize, y: usize) -> Option<&mut bool> {
        self.matrix.get_mut(y * self.width + x)
    }
    fn get_by_coordinate(&self, x: usize, y: usize) -> Option<&bool> {
        self.matrix.get(y * self.width + x)
    }

    fn mark_spot(&mut self, x: usize, y: usize, marked: bool) {
        *self.get_by_coordinate_mut(x, y).unwrap() = marked;
    }

    fn mark_spot_from_text(&mut self, text: &str) {
        let components: Vec<&str> = text.trim().split(',').collect();
        if components.len() != 2 {
            return;
        }
        self.mark_spot(
            components[0].parse().unwrap(),
            components[1].parse().unwrap(),
            true,
        );
    }

    fn unmark_fold_line(&mut self, fold: &Fold) {
        match fold {
            Fold::Vertical(y) => {
                for x in 0..self.width {
                    self.mark_spot(x, *y, false);
                }
            }
            Fold::Horizontal(x) => {
                for y in 0..self.height {
                    self.mark_spot(*x, y, false);
                }
            }
        }
    }

    fn translate_point_at_fold(
        &mut self,
        x: usize,
        y: usize,
        fold: &Fold,
    ) -> Option<(usize, usize)> {
        match fold {
            Fold::Vertical(y_fold) => {
                //check if point is above fold line, if so leave it
                if y < *y_fold {
                    return None;
                }
                let new_y_pos = y_fold - (y - y_fold);
                Some((x, new_y_pos))
            }
            Fold::Horizontal(x_fold) => {
                if x < *x_fold {
                    return None;
                }
                let new_x_pos = x_fold - (x - x_fold);
                Some((new_x_pos, y))
            }
        }
    }

    fn translate_all_points(&mut self, fold: &Fold) {
        self.unmark_fold_line(fold);
        for x in 0..self.width {
            for y in 0..self.height {
                // skip translation for unmarked points
                if !*self.get_by_coordinate(x, y).unwrap() {
                    continue;
                }
                if let Some((t_x, t_y)) = self.translate_point_at_fold(x, y, fold) {
                    self.mark_spot(t_x, t_y, true);
                    self.mark_spot(x, y, false);
                }
            }
        }
    }

    fn count_marked_spots(&self) -> usize {
        self.matrix.iter().filter(|&&y| y).count()
    }

    fn show(&self) {
        for i in 0..self.matrix.len() {
            if *self.matrix.get(i).unwrap() {
                print!("#");
            } else {
                print!(".");
            }
            if i > 0 && (i + 1) % self.width == 0 {
                print!("\n");
            }
        }
    }
}

#[derive(Debug)]
enum Fold {
    Vertical(usize),
    Horizontal(usize),
}

impl Fold {
    fn new(text: &str) -> Self {
        let components: Vec<&str> = text
            .strip_prefix("fold along ")
            .unwrap()
            .trim()
            .split('=')
            .collect();

        if components[0] == "y" {
            Fold::Vertical(components[1].parse().unwrap())
        } else {
            Fold::Horizontal(components[1].parse().unwrap())
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Provide the depth input text file!");
    }

    let file = File::open(&args[1]).expect("Error opening file");
    let reader = BufReader::new(file);

    let mut paper = Paper::new();
    let mut folds: Vec<Fold> = Vec::new();

    for line in reader.lines() {
        let text_line = line.unwrap();
        if text_line.starts_with("fold") {
            folds.push(Fold::new(&text_line));
        } else {
            paper.mark_spot_from_text(&text_line);
        }
    }

    println!("Folds: {:?}", folds);
    for fold in &folds {
        paper.translate_all_points(fold);
    }
    paper.show();
    println!("Points after folds: {}", paper.count_marked_spots());
}
