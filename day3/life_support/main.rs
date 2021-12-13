use std::fs;

fn bin_str_to_i32(bin_str: &str) -> i32 {
    let bin_iterator = bin_str.chars().rev();
    let mut multiplier = 1;
    let mut total = 0;

    for bit in bin_iterator {
        if bit == '1' {
            total += multiplier;
        }
        multiplier *= 2;
    }

    return total;
}

fn main() {
    let input_contents = fs::read_to_string("files/binary_diagnostic_input")
        .expect("Unable to read from input");

    let lines = input_contents.lines().collect::<Vec<&str>>();
    let num_bits = *(&lines[0].len()) as i32;

    let mut o2_gen_vec = Vec::new();
    let mut co2_scrub_vec = Vec::new();

    for line in lines {
        o2_gen_vec.push(line);
        co2_scrub_vec.push(line);
    }

    for bit in 0..num_bits as usize {
        let mut ones_counter = 0;

        // Count 1s in the current bit row
        for line in o2_gen_vec.clone() {
            if line.chars().nth(bit).unwrap() == '1' {
                ones_counter += 1;
            }
        }

        // Eliminate all lines with current bit not in the majority
        let num_lines_to_beat = (o2_gen_vec.len() as i32) / 2;
        let most_common_bit = if ones_counter >= num_lines_to_beat {'1'} else {'0'};
        o2_gen_vec.retain(|line| {
            return line.chars().nth(bit).unwrap() == most_common_bit;
        });

        if o2_gen_vec.len() == 1 {
            break;
        }
    }

    for bit in 0..num_bits as usize{
        let mut ones_counter = 0;

        // Count 1s in the current bit row
        for line in co2_scrub_vec.clone() {
            if line.chars().nth(bit).unwrap() == '1' {
                ones_counter += 1;
            }
        }

        // Eliminate all lines with current bit not in the minority
        let num_lines_to_beat = (co2_scrub_vec.len() as i32) / 2;
        let least_common_bit = if ones_counter < num_lines_to_beat {'1'} else {'0'};
        co2_scrub_vec.retain(|line| {
            return line.chars().nth(bit).unwrap() == least_common_bit;
        });

        if co2_scrub_vec.len() == 1 {
            break;
        }
    }

    let o2_gen_str: &str = o2_gen_vec[0];
    let co2_scrub_str: &str = co2_scrub_vec[0];

    let o2_gen_rate = bin_str_to_i32(o2_gen_str);
    let co2_scrub_rate = bin_str_to_i32(co2_scrub_str);

    let power_consumption = o2_gen_rate * co2_scrub_rate;
    println!("Oxygen Generation Rating: {} - {}", o2_gen_str, o2_gen_rate);
    println!("CO2 Scrub Rating: {} - {}", co2_scrub_str, co2_scrub_rate);
    println!("Life Support: {}", power_consumption);
}
