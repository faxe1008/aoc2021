use std::env;
use std::fs;

#[derive(Debug, Default, Clone)]
struct Octopus {
    abs_pos: usize,
    x: usize,
    y: usize,
    flash_counter: u8,
    has_flashed: bool,
}

impl Octopus {
    fn new(abs_pos: usize, x: usize, y: usize, flash_counter: u8) -> Self {
        Octopus {
            abs_pos,
            x,
            y,
            flash_counter,
            has_flashed: false,
        }
    }
    fn energize(&mut self) {
        self.flash_counter += 1;
    }
    fn does_flash(&self) -> bool {
        self.flash_counter > 9
    }
    fn reset_if_flashed(&mut self) {
        if self.does_flash() {
            self.flash_counter = 0;
        }
    }
}

#[derive(Debug)]
struct OctopusMap {
    width: usize,
    height: usize,
    octo_map: Vec<Octopus>,
}

impl OctopusMap {
    fn new(text: &str) -> Self {
        assert!(!text.is_empty());
        let height = text.lines().count();
        let width = text.lines().nth(0).unwrap().trim().len();
        let mut map: Vec<Octopus> = Vec::new();
        for (h, line) in text.lines().enumerate() {
            for (w, c) in line.trim().chars().enumerate() {
                let abs_pos = h * width + w;
                map.push(Octopus::new(abs_pos, w, h, c.to_digit(10).unwrap() as u8));
            }
        }
        OctopusMap {
            width,
            height,
            octo_map: map,
        }
    }

    fn get_octopus_by_coordinate(&self, x: isize, y: isize) -> Option<&Octopus> {
        if x < 0 || x >= self.width as isize {
            return None;
        }
        if y < 0 || y >= self.height as isize {
            return None;
        }
        self.octo_map.get(self.width * y as usize + x as usize)
    }

    fn get_neighbours(&self, x: usize, y: usize) -> Vec<&Octopus> {
        let mut neighbours = Vec::new();

        let offsets: [(isize, isize); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        for offset in offsets {
            let x_pos = offset.0 + x as isize;
            let y_pos = offset.1 + y as isize;
            if let Some(octo) = self.get_octopus_by_coordinate(x_pos, y_pos) {
                neighbours.push(octo);
            }
        }
        neighbours
    }

    fn energize_all(&mut self) {
        for octo in self.octo_map.iter_mut() {
            octo.energize();
        }
    }

    fn check_for_flashes(&mut self) -> usize {
        let mut total_flashes = 0;

        loop {
            let mut next_gen = self.octo_map.clone();
            // check for flashing octopi
            let flashing_octopi: Vec<Octopus> = self
                .octo_map
                .iter()
                .filter(|y| y.does_flash() && !y.has_flashed)
                .map(|y| y.clone())
                .collect();
            if flashing_octopi.is_empty() {
                break;
            }
            total_flashes += flashing_octopi.len();

            //mark the octos which have flashed acordingly
            for fl_oct in flashing_octopi.iter() {
                next_gen.get_mut(fl_oct.abs_pos).unwrap().has_flashed = true;
            }

            //get neighbours of all octopi that flashed
            let neighbours_of_flashing_octopi: Vec<&Octopus> = flashing_octopi
                .iter()
                .map(|octo| self.get_neighbours(octo.x, octo.y))
                .flatten()
                .collect();

            // all the neighbours of flashed octopi get enegergized as well
            for flashed_neighbour in &neighbours_of_flashing_octopi {
                next_gen
                    .get_mut(flashed_neighbour.abs_pos)
                    .unwrap()
                    .energize();
            }

            // next generation
            self.octo_map = next_gen;
        }

        for octo in self.octo_map.iter_mut() {
            if octo.has_flashed {
                octo.reset_if_flashed();
                octo.has_flashed = false;
            }
        }

        total_flashes
    }

    fn show(&self) {
        for octopus in &self.octo_map {
            if octopus.does_flash() {
                print!("\x1b[93m{:#2}\x1b[0m ", octopus.flash_counter);
            } else {
                print!("{:#2} ", octopus.flash_counter);
            }
            if (octopus.x + 1) % self.width == 0 {
                print!("\n");
            }
        }
    }

    fn all_flashed_synchronously(&self) -> bool {
        self.octo_map
            .iter()
            .filter(|y| y.flash_counter == 0)
            .count()
            == self.octo_map.len()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Provide the depth input text file!");
    }

    let text = fs::read_to_string(&args[1]).expect("Error reading file");
    let mut octo_map = OctopusMap::new(&text);

    let mut i = 0;
    loop {
        i += 1;
        octo_map.energize_all();
        octo_map.check_for_flashes();
        println!("Generation {}", i);
        octo_map.show();
        println!("");
        println!("");
        if octo_map.all_flashed_synchronously() {
            break;
        }
    }
}
