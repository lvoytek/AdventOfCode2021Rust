use std::fs;

fn main() {
    let input_contents = fs::read_to_string("files/lanternfish_init_input")
        .expect("Unable to read from input");

    let lines = input_contents.lines().collect::<Vec<&str>>();
    let fish_state_strs = lines[0].split(",").collect::<Vec<&str>>();
    let mut fish_state = Vec::<u32>::new();

    for fish in fish_state_strs {
        fish_state.push(fish.parse::<u32>().unwrap());
    }

    for _ in 0..80 {
        for i in 0..fish_state.len() {
            if fish_state[i] == 0 {
                fish_state[i] = 6;
                fish_state.push(8);
            }
            else {
                fish_state[i] -= 1;
            }
        }
    }

    println!("Number of fish: {}", fish_state.len());
}
