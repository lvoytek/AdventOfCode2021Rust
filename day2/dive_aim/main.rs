use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let input_file = File::open("files/dive_input")
        .expect("Unable to read from input");

    let lines = io::BufReader::new(input_file).lines();

    let mut horizontal_pos = 0;
    let mut vertical_pos = 0;
    let mut aim = 0;

    for line in lines {
        if let Ok(next_line) = line {
            let position_vector = next_line.split(" ").collect::<Vec<&str>>();
            let direction = position_vector[0];
            let int_value = position_vector[1].parse::<i32>().unwrap();

            match direction {
                "forward"=>{
                    horizontal_pos+=int_value;
                    vertical_pos+=aim*int_value;
                },
                "down"=>aim+=int_value,
                "up"=>aim-=int_value,
                _=>println!("Parse Error: Bad position"),
            }
        }
    }

    let multi_pos = horizontal_pos * vertical_pos;
    println!("Horizontal Position: {}", horizontal_pos);
    println!("Depth: {}", vertical_pos);
    println!("Depth x Horizontal Position: {}", multi_pos);
}
