use std::env;
use std::fs;

#[derive(Debug)]
struct CavernMap {
    width: usize,
    height: usize,
    risks: Vec<u8>,
}

#[derive(Debug)]
struct CavernPoint {
    x: usize,
    y: usize,
}

impl CavernMap {
    fn new(text: &str) -> Self {
        assert!(!text.is_empty());
        let height = text.lines().count();
        let width = text.lines().nth(0).unwrap().trim().len();

        let risks: Vec<u8> = text
            .lines()
            .map(|y| y.trim().chars())
            .flatten()
            .map(|y| y.to_digit(10).unwrap() as u8)
            .collect();

        CavernMap {
            width,
            height,
            risks,
        }
    }

    fn get_by_coordinate(&self, x: usize, y: usize) -> Option<&u8> {
        self.risks.get(y * self.width + x)
    }

    fn get_neighbours(&self, x: usize, y: usize) -> Vec<(CavernPoint, u8)> {
        let mut res = Vec::new();
        let offsets: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
        for offset in offsets {
            let new_x = x as isize + offset.0;
            let new_y = y as isize + offset.1;

            if new_x < 0
                || new_x >= self.width as isize
                || new_y < 0
                || new_y >= self.height as isize
            {
                continue;
            }
            res.push((
                CavernPoint {
                    x: new_x as usize,
                    y: new_y as usize,
                },
                *self
                    .get_by_coordinate(new_x as usize, new_y as usize)
                    .unwrap(),
            ));
        }
        res
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Provide the cavern input text file!");
    }

    let text = fs::read_to_string(&args[1]).expect("Error reading file");
    let cavern_map = CavernMap::new(&text);
    println!("{:?}", cavern_map.get_neighbours(1, 0));
}
