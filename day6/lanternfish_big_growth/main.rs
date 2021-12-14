use std::fs;

// Recursively get offspring number given days left and days before first double
fn get_num_fish_descendants(num_days_left: i32, initial_state: i32) -> u64 {
    // Start with current fish
    let mut total_fish: u64 = 1;

    let mut day_countdown: i32 = num_days_left - initial_state;

    while day_countdown > 0 {
        total_fish += get_num_fish_descendants(day_countdown, 9);
        day_countdown -= 7;
    }

    return total_fish;
}

fn main() {
    let num_days = 256;

    let input_contents = fs::read_to_string("files/lanternfish_init_input")
        .expect("Unable to read from input");

    let lines = input_contents.lines().collect::<Vec<&str>>();
    let fish_state_strs = lines[0].split(",").collect::<Vec<&str>>();
    let mut fish_state = Vec::<i32>::new();

    for fish in fish_state_strs {
        fish_state.push(fish.parse::<i32>().unwrap());
    }

    // Bin each fish to wait times so each is only calculated once
    let longest_fish_wait: i32 = *fish_state.iter().max().unwrap();
    let mut fish_bins = vec![0; (longest_fish_wait + 1) as usize];

    for fish_wait in fish_state {
        fish_bins[fish_wait as usize] += 1;
    }

    let mut total_fish = 0;

    for i in 0..=longest_fish_wait {
        if fish_bins[i as usize] > 0 {
            total_fish += get_num_fish_descendants(num_days, i) * fish_bins[i as usize];
            println!("{}", total_fish)
        }
    }

    println!("Number of fish: {}", total_fish);
}
