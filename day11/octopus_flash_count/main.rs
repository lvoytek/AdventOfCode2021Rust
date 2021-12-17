use std::fs;

fn get_flashes(i: usize, j: usize, octomap: &mut Vec<Vec<i32>>) -> u32 {
    // Check if current octopus should flash, otherwise exit early
    if octomap[i][j] > 9 {
        octomap[i][j] = 0;

        let mut flash_count = 1;

        // Increase adjacent values if they have not just flashed
        if octomap[i-1][j-1] > 0 {
            octomap[i-1][j-1] += 1;
        }

        if octomap[i][j-1] > 0 {
            octomap[i][j-1] += 1;
        }

        if octomap[i+1][j-1] > 0 {
            octomap[i+1][j-1] += 1;
        }

        if octomap[i+1][j] > 0 {
            octomap[i+1][j] += 1;
        }

        if octomap[i+1][j+1] > 0 {
            octomap[i+1][j+1] += 1;
        }

        if octomap[i][j+1] > 0 {
            octomap[i][j+1] += 1;
        }

        if octomap[i-1][j+1] > 0 {
            octomap[i-1][j+1] += 1;
        }

        if octomap[i-1][j] > 0 {
            octomap[i-1][j] += 1;
        }

        // Recurse into adjacent areas
        flash_count += get_flashes(i-1, j-1, octomap);
        flash_count += get_flashes(i, j-1, octomap);
        flash_count += get_flashes(i+1, j-1, octomap);
        flash_count += get_flashes(i+1, j, octomap);
        flash_count += get_flashes(i+1, j+1, octomap);
        flash_count += get_flashes(i, j+1, octomap);
        flash_count += get_flashes(i-1, j+1, octomap);
        flash_count += get_flashes(i-1, j, octomap);

        return flash_count;
    }

    return 0;
}

fn main() {
    let input_contents = fs::read_to_string("files/octopus_energy_level_input")
        .expect("Unable to read from input");

    let lines = input_contents.lines().collect::<Vec<&str>>();
    let width = lines[0].len();
    let height = lines.len();

    // Build numeric matrix with max val outline
    const RADIX: u32 = 10;
    let mut octomap = Vec::<Vec::<i32>>::new();

    octomap.push(Vec::<i32>::new());
    for _ in 0..width + 2 {
        octomap[0].push(-1);
    }

    for line in lines {
        octomap.push(Vec::<i32>::new());
        let current_vec_index = (octomap.len() - 1) as usize;

        octomap[current_vec_index].push(-1);
        for val in line.chars() {
            octomap[current_vec_index].push(val.to_digit(RADIX).unwrap() as i32);
        }
        octomap[current_vec_index].push(-1);
    }

    octomap.push(Vec::<i32>::new());
    let current_vec_index = (octomap.len() - 1) as usize;
    for _ in 0..width + 2 {
        octomap[current_vec_index].push(-1);
    }

    let mut flash_count = 0;

    // Simulate 100 days
    for _ in 0..100 {
        // Increase all values by 1
        for i in 1..=height {
            for j in 1..=width {
                octomap[i][j] += 1;
            }
        }

        // Go through flashes
        for i in 1..=height {
            for j in 1..=width {
                flash_count += get_flashes(i, j, &mut octomap);
            }
        }
    }

    println!("Flash count: {}", flash_count);
}
