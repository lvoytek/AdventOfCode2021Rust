use std::fs;

fn main() {
    let input_contents = fs::read_to_string("files/crab_position_input")
        .expect("Unable to read from input");

    let lines = input_contents.lines().collect::<Vec<&str>>();
    let crab_pos_strs = lines[0].split(",").collect::<Vec<&str>>();
    let mut crab_posns = Vec::<i32>::new();

    for crab_str in crab_pos_strs {
        crab_posns.push(crab_str.parse::<i32>().unwrap());
    }

    // Find the median location
    crab_posns.sort();
    let midpoint = crab_posns[(crab_posns.len() / 2) as usize];

    // Get fuel needed to move to mean location
    let mut total_fuel = 0;

    for crab_pos in crab_posns {
        total_fuel += (midpoint - crab_pos).abs();
    }

    println!("Fuel needed: {}", total_fuel);
}
