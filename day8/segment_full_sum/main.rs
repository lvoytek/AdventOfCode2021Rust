use std::fs;

#[derive(Copy, Clone)]
struct DigitCal {
    top: char,
    top_left: char,
    top_right: char,
    middle: char,
    bottom_left: char,
    bottom_right: char,
    bottom: char,
}

fn print_digit_cal(digit_cal: &DigitCal) {
    print!(" ");
    for _ in 0..4 {
        print!("{}", digit_cal.top);
    }
    println!("");

    println!("{}    {}", digit_cal.top_left, digit_cal.top_right);
    println!("{}    {}", digit_cal.top_left, digit_cal.top_right);

    print!(" ");
    for _ in 0..4 {
        print!("{}", digit_cal.middle);
    }
    println!("");

    println!("{}    {}", digit_cal.bottom_left, digit_cal.bottom_right);
    println!("{}    {}", digit_cal.bottom_left, digit_cal.bottom_right);

    print!(" ");
    for _ in 0..4 {
        print!("{}", digit_cal.bottom);
    }
    println!("");
    println!("");
}

fn calibrate_digit(calibration_values: &Vec<&str>) -> DigitCal {
    let mut digit_cal = DigitCal {
        top : '.',
        top_left: '.',
        top_right: '.',
        middle: '.',
        bottom_left: '.',
        bottom_right: '.',
        bottom: '.',
    };

    let mut char_list = vec!{'a', 'b', 'c', 'd', 'e', 'f', 'g'};

    // Start by finding the 1
    let mut one_str: &str = "";

    for val in calibration_values {
        if val.len() == 2 {
            one_str = val;
            break;
        }
    }

    if one_str.len() != 2 {
        println!("No 1 found in calibration");
        return digit_cal;
    }

    // Next find the 7
    let mut seven_str: &str = "";

    for val in calibration_values {
        if val.len() == 3 {
            seven_str = val;
            break;
        }
    }

    if seven_str.len() != 3 {
        println!("No 7 found in calibration");
        return digit_cal;
    }

    // Determine the top character
    for letter in seven_str.chars() {
        if !one_str.contains(letter) {
            digit_cal.top = letter;
            char_list.retain(|l| { return l != &letter; });
            break;
        }
    }

    // Find the 4
    let mut four_str: &str = "";

    for val in calibration_values {
        if val.len() == 4 {
            four_str = val;
            break;
        }
    }

    if four_str.len() != 4 {
        println!("No 4 found in calibration");
        return digit_cal;
    }

    // Find the 6 - size 6 + one of the 1 components is missing
    // And the 9 - size 6 + contains all of 4's values
    // And the 0 - size 6 but not the 6 or the 9
    // This will determine the right side vals at the same time
    let mut six_str: &str = "";
    let mut nine_str: &str = "";
    let mut zero_str: &str = "";
    let right_letter_1 = one_str.chars().nth(0).unwrap();
    let right_letter_2 = one_str.chars().nth(1).unwrap();

    for val in calibration_values {
        if val.len() == 6 {
            if !val.contains(right_letter_1) {
                six_str = val;
                digit_cal.top_right = right_letter_1;
                digit_cal.bottom_right = right_letter_2;
                char_list.retain(|l| { return l != &right_letter_1 && l != &right_letter_2; });
            }
            else if !val.contains(right_letter_2) {
                six_str = val;
                digit_cal.top_right = right_letter_2;
                digit_cal.bottom_right = right_letter_1;
                char_list.retain(|l| { return l != &right_letter_1 && l != &right_letter_2; });
            }
            else {
                // 9 already found - this is 0
                if nine_str.len() == 6 {
                    zero_str = val;
                }

                // 0 already found - this is 9
                else if zero_str.len() == 6 {
                    nine_str = val;
                }

                // Determine whether the val is 0 or 9
                else {
                    for letter in four_str.chars() {
                        if !val.contains(letter) {
                            zero_str = val;
                            break;
                        }
                    }

                    if zero_str.len() != 6 {
                        nine_str = val;
                    }
                }
            }
        }
    }

    if six_str.len() != 6 {
        println!("No 6 found in calibration");
        return digit_cal;
    }

    if zero_str.len() != 6 {
        println!("No 0 found in calibration");
        return digit_cal;
    }

    if nine_str.len() != 6 {
        println!("No 9 found in calibration");
        return digit_cal;
    }

    // Find the middle - in 4 but not 0
    for letter in four_str.chars() {
        if !zero_str.contains(letter) {
            digit_cal.middle = letter;
            char_list.retain(|l| { return l != &letter; });
            break;
        }
    }

    // Find the 3 - length 5 containing both 1 chars
    let mut three_str: &str = "";

    for val in calibration_values {
        if val.len() == 5 {
            if val.contains(right_letter_1) && val.contains(right_letter_2) {
                three_str = val;
                break;
            }
        }
    }

    if three_str.len() != 5 {
        println!("No 3 found in calibration");
        return digit_cal;
    }

    // Find top left - in 9 but not 3
    for letter in nine_str.chars() {
        if !three_str.contains(letter) {
            digit_cal.top_left = letter;
            char_list.retain(|l| { return l != &letter; });
            break;
        }
    }

    // Find the bottom - only char in 9 not accounted for
    for letter in nine_str.chars() {
        if letter != digit_cal.top && letter != digit_cal.top_left &&
           letter != digit_cal.top_right && letter != digit_cal.middle && 
           letter != digit_cal.bottom_right {
            digit_cal.bottom = letter;
            char_list.retain(|l| { return l != &letter; });
            break;
        }
    }

    // Find the bottom left - the only remaining letter
    digit_cal.bottom_left = char_list[0];

    return digit_cal;
}

fn extract_digit(digit_str: &str, calibration: DigitCal) -> i32 {
    match digit_str.len() {
        2 => return 1,
        3 => return 7,
        4 => return 4,
        5 => {
            if digit_str.contains(calibration.top_left) {
                return 5;
            }
            else if digit_str.contains(calibration.bottom_left) {
                return 2;
            }
            else {
                return 3;
            }
        },
        6 => {
            if !digit_str.contains(calibration.middle) {
                return 0;
            }
            else if digit_str.contains(calibration.bottom_left) {
                return 6;
            }
            else {
                return 9;
            }
        },
        7 => return 8,
        _ => return -1,
    }
}

fn main() {
    let input_contents = fs::read_to_string("files/7_segment_input")
        .expect("Unable to read from input");

    let lines = input_contents.lines().collect::<Vec<&str>>();

    let mut overall_count = 0;

    for line in lines {
        let split_input = line.split(" | ").collect::<Vec<&str>>();
        let calibration_values = split_input[0].split_whitespace().collect::<Vec<&str>>();
        let digit_values = split_input[1].split_whitespace().collect::<Vec<&str>>();

        let digit_cal = calibrate_digit(&calibration_values);
        let mut current_val = 0;
        let mut multiplier = 1000;

        for digit in digit_values {
            current_val += extract_digit(digit, digit_cal) * multiplier;
            multiplier /= 10;
        }

        overall_count += current_val;
    }

    println!("Overall count: {}", overall_count);
}
