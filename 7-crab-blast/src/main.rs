use std::env;
use std::fs;

fn mean(list: &[u32]) -> f64 {
    let sum: u32 = list.iter().sum();
    sum as f64 / list.len() as f64
}

fn median(list: &[u32]) -> f64 {
    if (list.len() % 2) == 0 {
        let ind_left = list.len() / 2 - 1;
        let ind_right = list.len() / 2;
        (list[ind_left] + list[ind_right]) as f64 / 2.0
    } else {
        list[(list.len() / 2)] as f64
    }
}

fn get_fuel_for_pos(list: &[u32], pos: u32) -> u32 {
    list.iter()
        .map(|&y| {
            let distance = (y as i64 - pos as i64).abs();
            ((distance.pow(2) + distance) / 2) as u32
        })
        .sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Provide the depth input text file!");
    }

    let text = fs::read_to_string(&args[1]).expect("Error reading file");
    let mut crabs_positions: Vec<u32> =
        text.trim().split(',').map(|y| y.parse().unwrap()).collect();
    crabs_positions.sort();

    let max_distance = *crabs_positions.iter().max().unwrap();
    let mut cheapest_distance = 0;
    let mut lowest_fuel_cost = u32::max_value();
    for i in 0..max_distance {
        let fuel_cost = get_fuel_for_pos(&crabs_positions, i);
        println!("{}: {} cost", i, fuel_cost);
        if fuel_cost < lowest_fuel_cost {
            cheapest_distance = i;
            lowest_fuel_cost = fuel_cost;
        }
    }

    println!(
        "Chepeast distance: {}, fuel: {}",
        cheapest_distance, lowest_fuel_cost
    );
}
