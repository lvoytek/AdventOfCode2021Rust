use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let input_file = File::open("files/sonar_input")
        .expect("Unable to read from input");

    let lines = io::BufReader::new(input_file).lines();

    let mut values = Vec::new();
    for line in lines {
        if let Ok(next_line) = line {
            let int_value = next_line.parse::<i32>().unwrap();
            values.push(int_value);
        }
    }

    let mut larger_count = 0;
    let mut prev_sum = i32::MAX;

    for i in 0..values.len() - 2 {
        let new_sum = &values[i] + &values[i+1] + &values[i+2];

        if new_sum > prev_sum {
            larger_count += 1;
        }

        prev_sum = new_sum;
    }

    println!("Number of sum increases: {}", larger_count)


}
