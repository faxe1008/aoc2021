use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const VENT_FIELD_SIZE: usize = 1000;

#[derive(Default, Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Default, Debug)]
struct VentPath {
    start: Point,
    end: Point,
}

impl VentPath {
    fn new(text: &str) -> Result<VentPath, &'static str> {
        let coordinates: Vec<&str> = text.split("->").map(|x| x.trim()).collect();
        if coordinates.len() != 2 {
            return Err("Param count");
        }

        let start_coordinates: Vec<u32> = coordinates[0]
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        if start_coordinates.len() != 2 {
            return Err("Param count");
        }
        let start = Point {
            x: start_coordinates[0],
            y: start_coordinates[1],
        };

        let end_coordinates: Vec<u32> = coordinates[1]
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        if end_coordinates.len() != 2 {
            return Err("Param count");
        }
        let end = Point {
            x: end_coordinates[0],
            y: end_coordinates[1],
        };

        Ok(VentPath { start, end })
    }
}

#[derive(Debug)]
struct VentField {
    vents: [u32; VENT_FIELD_SIZE * VENT_FIELD_SIZE],
}

impl VentField {
    fn new() -> VentField {
        VentField {
            vents: [0; VENT_FIELD_SIZE * VENT_FIELD_SIZE],
        }
    }

    fn mark_single_field(&mut self, x: u32, y: u32) {
        self.vents[y as usize * VENT_FIELD_SIZE + x as usize] += 1;
    }

    fn mark_path(&mut self, path: &VentPath) {
        let x_distance = path.end.x as i32 - path.start.x as i32;
        let y_distance = path.end.y as i32 - path.start.y as i32;
        let y_range = if y_distance < 0 {
            path.end.y..=path.start.y
        } else {
            path.start.y..=path.end.y
        };
        let x_range = if x_distance < 0 {
            path.end.x..=path.start.x
        } else {
            path.start.x..=path.end.x
        };

        if x_distance.abs() == y_distance.abs() {
            for x in x_range {
                for y in y_range.clone() {
                    let point_x_dist: u32 = (path.start.x as i32 - x as i32).abs() as u32;
                    let point_y_dist: u32 = (path.start.y as i32 - y as i32).abs() as u32;
                    if point_x_dist == point_y_dist {
                        self.mark_single_field(x, y);
                    }
                }
            }
        } else if x_distance == 0 {
            for y in y_range {
                //eprintln!("Marking {},{} for {:?}", path.start.x, y, path);
                self.mark_single_field(path.start.x, y);
            }
        } else if y_distance == 0 {
            for x in x_range {
                //eprintln!("Marking {},{} for {:?}", x, path.start.y, path);
                self.mark_single_field(x, path.start.y);
            }
        } else {
            eprintln!("Invalid path: {:?}", path);
        }
    }

    fn get_dangerous_field_count(&self) -> usize {
        self.vents.iter().filter(|&&y| y >= 2).count()
    }

    fn print(&self) {
        for i in 0..self.vents.len() {
            print!("{}", self.vents[i]);
            if i != 0 && (i as i32 - VENT_FIELD_SIZE as i32 + 1) % VENT_FIELD_SIZE as i32 == 0 {
                print!("\n");
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Provide the vents input text file!");
    }

    let file = File::open(&args[1]).expect("Error opening file");
    let reader = BufReader::new(file);

    let paths = reader
        .lines()
        .map(|x| VentPath::new(&x.unwrap()).expect("Error parsing ventpath"))
        .collect::<Vec<VentPath>>();

    let mut field = VentField::new();
    for path in paths {
        field.mark_path(&path);
    }
    println!(
        "Dangerous field count: {}",
        field.get_dangerous_field_count()
    );
}
