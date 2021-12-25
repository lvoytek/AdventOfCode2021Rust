use std::fs;

fn main() {
    let input_contents = fs::read_to_string("files/snailfish_num_input")
        .expect("Unable to read from input");

    let lines = input_contents.lines().collect::<Vec<&str>>();
    
}
