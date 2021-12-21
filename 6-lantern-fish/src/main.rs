use std::env;
use std::fs;

#[derive(Clone)]
struct LanternFish {
    reproduction_timer: u32,
}

impl Default for LanternFish {
    fn default() -> Self {
        LanternFish {
            reproduction_timer: 8,
        }
    }
}

impl std::fmt::Debug for LanternFish {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        print!("{}", self.reproduction_timer);
        Ok(())
    }
}

impl LanternFish {
    fn new(timer: u32) -> Self {
        LanternFish {
            reproduction_timer: timer,
        }
    }
    fn tick_day(&mut self) -> Option<LanternFish> {
        if self.reproduction_timer == 0 {
            self.reproduction_timer = 6;
            Some(LanternFish::default())
        } else {
            self.reproduction_timer -= 1;
            None
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Provide the depth input text file!");
    }

    let text = fs::read_to_string(&args[1]).expect("Error reading file");
    let mut fish_swarm: Vec<LanternFish> = text
        .trim()
        .split(',')
        .map(|x| LanternFish::new(x.parse().expect("Error converting to int")))
        .collect();
    let generations: usize = 80;

    for i in 1..=generations {
        let mut next_state: Vec<LanternFish> = Vec::new();
        for fish in fish_swarm.iter_mut() {
            if let Some(new_fish) = fish.tick_day() {
                next_state.push(new_fish);
            }
            next_state.push(fish.clone());
        }
        println!("Day {}: {:?}", i, next_state);
        fish_swarm = next_state.clone();
    }
    println!("There are {} fish", fish_swarm.len());
}
