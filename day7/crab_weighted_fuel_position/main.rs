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

    let min_position = *crab_posns.iter().min().unwrap();
    let max_position = *crab_posns.iter().max().unwrap();

    // Get fuel needed to move to best location
    let mut min_total_fuel = i32::MAX;

    for midpoint in min_position..=max_position {
        let mut total_fuel = 0;

        for crab_pos in &crab_posns {
            let distance = (midpoint - crab_pos).abs();
            total_fuel += (1 + distance) * distance / 2;
        }

        if total_fuel < min_total_fuel {
            min_total_fuel = total_fuel;
        }
    }

    println!("Fuel needed: {}", min_total_fuel);
}
