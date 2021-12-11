use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let input_file = File::open("files/sonar_input")
        .expect("Unable to read from input");

    let lines = io::BufReader::new(input_file).lines();

    let mut larger_count = 0;
    let mut prev_value = i32::MAX;
    for line in lines {
        if let Ok(next_line) = line {
            let int_value = next_line.parse::<i32>().unwrap();
            if int_value > prev_value {
                larger_count += 1;
            }

            prev_value = int_value;
        }
    }

    println!("Number of increases: {}", larger_count)
}
