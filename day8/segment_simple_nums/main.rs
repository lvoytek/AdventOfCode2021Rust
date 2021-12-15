use std::fs;

fn main() {
    let input_contents = fs::read_to_string("files/7_segment_input")
        .expect("Unable to read from input");

    let lines = input_contents.lines().collect::<Vec<&str>>();

    let mut digit_counter_array: [u32; 10] = [0; 10];
    let mut other_counter = 0;

    for line in lines {
        let split_input = line.split(" | ").collect::<Vec<&str>>();
        let digit_values = split_input[1].split_whitespace().collect::<Vec<&str>>();

        for digit_value in digit_values {
            match digit_value.len() {
                2 => digit_counter_array[1] += 1,
                4 => digit_counter_array[4] += 1,
                3 => digit_counter_array[7] += 1,
                7 => digit_counter_array[8] += 1,
                _ => other_counter += 1,
            }
        }
    }

    println!("Total simple numbers: {}", digit_counter_array.iter().sum::<u32>());
}
