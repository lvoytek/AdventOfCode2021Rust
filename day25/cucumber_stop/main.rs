use std::fs;

#[derive(PartialEq, Clone, Copy)]
enum CucumberState {
    East,
    South,
    None,
    Moving,
}

// Return true if there was no movement
fn step(cucumber_map: &mut Vec<Vec<CucumberState>>) -> bool {
    let mut no_movement = true;

    // Move east
    for i in (0..cucumber_map.len()).rev() {
        for j in (0..cucumber_map[0].len()).rev() {
            if cucumber_map[i][j] == CucumberState::East {
                if j == cucumber_map[0].len() - 1 {
                    if cucumber_map[i][0] == CucumberState::None {
                        cucumber_map[i][j] = CucumberState::Moving;
                        no_movement = false;
                    }
                }
                else {
                    if cucumber_map[i][j + 1] == CucumberState::None {
                        cucumber_map[i][j + 1] = CucumberState::East;
                        cucumber_map[i][j] = CucumberState::Moving;
                        no_movement = false;
                    }
                }
            }
        }
    }

    for i in 0..cucumber_map.len() {
        for j in 0..cucumber_map[0].len() {
            if cucumber_map[i][j] == CucumberState::Moving {
                cucumber_map[i][j] = CucumberState::None;

                if j == cucumber_map[0].len() - 1 {
                    cucumber_map[i][0] = CucumberState::East;
                }
            }
        }
    }

    // Move south
    for i in (0..cucumber_map.len()).rev() {
        for j in (0..cucumber_map[0].len()).rev() {
            if cucumber_map[i][j] == CucumberState::South {
                if i == cucumber_map.len() - 1 {
                    if cucumber_map[0][j] == CucumberState::None {
                        cucumber_map[i][j] = CucumberState::Moving;
                        no_movement = false;
                    }
                }
                else {
                    if cucumber_map[i + 1][j] == CucumberState::None {
                        cucumber_map[i + 1][j] = CucumberState::South;
                        cucumber_map[i][j] = CucumberState::Moving;
                        no_movement = false;
                    }
                }
            }
        }
    }

    for i in 0..cucumber_map.len() {
        for j in 0..cucumber_map[0].len() {
            if cucumber_map[i][j] == CucumberState::Moving {
                cucumber_map[i][j] = CucumberState::None;

                if i == cucumber_map.len() - 1 {
                    cucumber_map[0][j] = CucumberState::South;
                }
            }
        }
    }

    return no_movement;
}

fn main() {
    let input_contents = fs::read_to_string("files/cucumber_state_input")
        .expect("Unable to read from input");

    let lines = input_contents.lines().collect::<Vec<&str>>();

    let mut cucumber_map = Vec::<Vec::<CucumberState>>::new();

    for line in lines {
        cucumber_map.push(Vec::<CucumberState>::new());
        let newest_row = cucumber_map.len() - 1;
        for slot in line.chars() {
            match slot {
                '>' => cucumber_map[newest_row].push(CucumberState::East),
                'v' => cucumber_map[newest_row].push(CucumberState::South),
                '.' => cucumber_map[newest_row].push(CucumberState::None),
                _ => println!("Error, Invalid character: {}", slot),
            }
        }
    }

    let mut step_count: u32 = 1;

    while !step(&mut cucumber_map) {
        step_count += 1;
    }

    println!("Num steps: {}", step_count);

}
