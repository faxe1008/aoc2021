use std::env;
use std::fs;

#[derive(Debug)]
struct SmokeBasin {
    width: usize,
    height: usize,
    height_map: Vec<u8>,
}

impl SmokeBasin {
    fn new(text: &str) -> Self {
        assert!(!text.is_empty());
        let height = text.lines().count();
        let width = text.lines().nth(0).unwrap().trim().len();
        let mut height_map: Vec<u8> = Vec::new();
        for line in text.lines() {
            for c in line.chars() {
                height_map.push(c.to_digit(10).unwrap() as u8);
            }
        }
        SmokeBasin {
            width,
            height,
            height_map,
        }
    }

    fn get_neighbours(&self, pos: usize) -> Vec<u8> {
        let mut neighbours = Vec::new();

        //left neighbour
        if pos > 0 {
            neighbours.push(self.height_map[pos - 1]);
        }
        //right neighbour
        if pos < self.height_map.len() - 1 {
            neighbours.push(self.height_map[pos + 1]);
        }
        //upper neighbour
        let upper_neighbour_pos = pos as isize - self.width as isize;
        if upper_neighbour_pos >= 0 {
            neighbours.push(self.height_map[upper_neighbour_pos as usize]);
        }
        //lower neighbour
        let lower_neighbour_pos = pos + self.width;
        if lower_neighbour_pos < self.height_map.len() {
            neighbours.push(self.height_map[lower_neighbour_pos]);
        }
        neighbours
    }

    fn get_local_height_minima(&self) -> Vec<u8> {
        let mut minima = Vec::new();
        for (index, value) in self.height_map.iter().enumerate() {
            let lower_or_same_height_neighbours_count: usize = self
                .get_neighbours(index)
                .iter()
                .filter(|&y| y <= value)
                .count();
            if lower_or_same_height_neighbours_count == 0 {
                minima.push(*value);
            }
        }
        minima
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Provide the depth input text file!");
    }

    let text = fs::read_to_string(&args[1]).expect("Error reading file");
    let smoke_basin = SmokeBasin::new(&text);

    let comined_risk_level: u32 = smoke_basin
        .get_local_height_minima()
        .iter()
        .map(|y| (y + 1) as u32)
        .sum();
    println!("Combined risk level {}", comined_risk_level);
}
