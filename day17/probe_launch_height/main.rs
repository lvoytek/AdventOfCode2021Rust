use std::fs;

struct Target {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

fn is_on_target(x: i32, y: i32, target: &Target) -> bool{
    return x >= target.x_min && x <= target.x_max && y >= target.y_min && y <= target.y_max;
}

// Returns max height if target hit, 0 otherwise
fn get_max_height(initial_x_vel: i32, initial_y_vel: i32, target: &Target) -> i32 {
    let mut x_vel = initial_x_vel;
    let mut y_vel = initial_y_vel;
    let mut x = 0;
    let mut y = 0;
    let mut max_height = 0;

    loop {
        if is_on_target(x, y, target) {
            return max_height;
        }

        if y < target.y_min {
            return 0;
        }

        x += x_vel;
        y += y_vel;

        if y > max_height {
            max_height = y;
        }

        if x_vel > 0 {
            x_vel -= 1;
        }
        else if x_vel < 0 {
            x_vel += 1;
        }

        y_vel -= 1;
    }

}

fn main() {
    let input_contents = fs::read_to_string("files/probe_launch_input")
        .expect("Unable to read from input");

    let lines = input_contents.lines().collect::<Vec<&str>>();
    let launch_input = lines[0];

    // Get target coordinates
    let launch_values = launch_input.split("x=").collect::<Vec<&str>>()[1].split(", y=").collect::<Vec<&str>>();
    let x_strs = launch_values[0].split("..").collect::<Vec<&str>>();
    let y_strs = launch_values[1].split("..").collect::<Vec<&str>>();

    let target = Target {
        x_min: x_strs[0].parse::<i32>().unwrap(),
        x_max: x_strs[1].parse::<i32>().unwrap(),
        y_min: y_strs[0].parse::<i32>().unwrap(),
        y_max: y_strs[1].parse::<i32>().unwrap(),
    };

    let mut max_height = 0;

    // Determine valid trajectory with highest max height
    for x_vel in 0..target.x_max {
        for y_vel in target.y_min..(-2 * target.y_min) {
            let max_round_height = get_max_height(x_vel, y_vel, &target);

            if max_round_height > max_height {
                max_height = max_round_height;
            }
        }
    }

    println!("Max Height: {}", max_height);

}
