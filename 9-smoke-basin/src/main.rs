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

    fn get_neighbours(&self, pos: usize) -> Vec<(usize, u8)> {
        let mut neighbours = Vec::new();

        //left neighbour
        if pos > 0 && pos % self.width != 0 {
            neighbours.push((pos - 1, self.height_map[pos - 1]));
        }
        //right neighbour
        if pos < self.height_map.len() - 1 && (pos + 1) % self.width != 0 {
            neighbours.push((pos + 1, self.height_map[pos + 1]));
        }
        //upper neighbour
        let upper_neighbour_pos = pos as isize - self.width as isize;
        if upper_neighbour_pos >= 0 {
            neighbours.push((
                upper_neighbour_pos as usize,
                self.height_map[upper_neighbour_pos as usize],
            ));
        }
        //lower neighbour
        let lower_neighbour_pos = pos + self.width;
        if lower_neighbour_pos < self.height_map.len() {
            neighbours.push((
                lower_neighbour_pos as usize,
                self.height_map[lower_neighbour_pos],
            ));
        }
        neighbours
    }

    fn get_local_height_minima(&self) -> Vec<(usize, u8)> {
        let mut minima = Vec::new();
        for (index, value) in self.height_map.iter().enumerate() {
            let lower_or_same_height_neighbours_count: usize = self
                .get_neighbours(index)
                .iter()
                .filter(|&(index, height)| height <= value)
                .count();
            if lower_or_same_height_neighbours_count == 0 {
                minima.push((index, *value));
            }
        }
        minima
    }

    fn get_non_wall_neighbours(&self, pos: usize) -> Vec<usize> {
        self.get_neighbours(pos)
            .iter()
            .filter(|&(_, height)| *height != 9)
            .map(|(pos, _)| *pos)
            .collect()
    }

    fn get_basins(&self) -> Vec<Vec<usize>> {
        let mut basins = Vec::new();

        let local_minima = self.get_local_height_minima();
        for minima in local_minima {
            println!("Working on minima: {:?}", minima);
            let mut basin = vec![minima.0];

            let mut neighbour_stack = self.get_non_wall_neighbours(minima.0);
            basin.append(&mut neighbour_stack.clone());
            while let Some(neighbour_pos) = neighbour_stack.pop() {
                basin.push(neighbour_pos);
                let mut new_cells = self
                    .get_non_wall_neighbours(neighbour_pos)
                    .iter()
                    .filter(|y| !basin.contains(&y) && !neighbour_stack.contains(y))
                    .map(|y| *y)
                    .collect();
                neighbour_stack.append(&mut new_cells);
            }

            basin.sort();
            basin.dedup();
            basins.push(basin);
        }
        basins
    }

    fn show(&self, basin: &Vec<usize>) {
        for (index, value) in self.height_map.iter().enumerate() {
            if basin.contains(&index) {
                print!("\x1b[93m{:#2}\x1b[0m ", value);
            } else {
                print!("{:#2} ", value);
            }
            if (index + 1) % self.width == 0 {
                print!("\n");
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Provide the depth input text file!");
    }

    let text = fs::read_to_string(&args[1]).expect("Error reading file");
    let smoke_basin = SmokeBasin::new(&text);
    let basins = smoke_basin.get_basins();
    let mut basin_sizes: Vec<usize> = basins.iter().map(|y| y.len()).collect();
    basin_sizes.sort();
    println!("{:?}", basin_sizes);
}
