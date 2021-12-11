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
    let num_lines = (lines.len() as i32) / 2;

    let mut ones_counter = Vec::new();

    for _ in 0..num_bits {
        ones_counter.push(0);
    }

    for line in lines {
        let mut bit_loc = 0;
        for bit in line.chars() {
            if bit == '1' {
                ones_counter[bit_loc] += 1;
            }

            bit_loc += 1;
        }
    }

    let mut gamma_string: String = "".to_owned();
    let mut epsilon_string: String = "".to_owned();

    for bit in ones_counter as Vec<i32>{
        if bit > num_lines {
            gamma_string.push_str("1");
            epsilon_string.push_str("0");
        }
        else {
            gamma_string.push_str("0");
            epsilon_string.push_str("1");
        }
    }

    let gamma_str: &str = &gamma_string;
    let epsilon_str: &str = &epsilon_string;

    let gamma_rate = bin_str_to_i32(gamma_str);
    let epsilon_rate = bin_str_to_i32(epsilon_str);

    let power_consumption = gamma_rate * epsilon_rate;
    println!("Gamma Rate: {} - {}", gamma_str, gamma_rate);
    println!("Epsilon Rate: {} - {}", epsilon_str, epsilon_rate);
    println!("Power Consumtion: {}", power_consumption);
}
