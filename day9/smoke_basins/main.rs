use std::fs;

fn get_basin_size(i: usize, j: usize, heightmap: &mut Vec<Vec<u32>>) -> u32 {
    // Include current spot and set it to 9 to mark it done
    let mut basin_size = 1;
    heightmap[i][j] = 9;

    if heightmap[i-1][j] < 9 {
        basin_size += get_basin_size(i-1, j, heightmap);
    }

    if heightmap[i+1][j] < 9 {
        basin_size += get_basin_size(i+1, j, heightmap);
    }

    if heightmap[i][j-1] < 9 {
        basin_size += get_basin_size(i, j-1, heightmap);
    }

    if heightmap[i][j + 1] < 9 {
        basin_size += get_basin_size(i, j+1, heightmap);
    }

    return basin_size;

}

fn main() {
    let input_contents = fs::read_to_string("files/heightmap_input")
        .expect("Unable to read from input");

    let lines = input_contents.lines().collect::<Vec<&str>>();
    let width = lines[0].len();
    let height = lines.len();

    // Build numeric matrix with max val outline
    const RADIX: u32 = 10;
    let mut heightmap = Vec::<Vec::<u32>>::new();

    heightmap.push(Vec::<u32>::new());
    for _ in 0..width + 2 {
        heightmap[0].push(9);
    }

    for line in lines {
        heightmap.push(Vec::<u32>::new());
        let current_vec_index = (heightmap.len() - 1) as usize;

        heightmap[current_vec_index].push(9);
        for val in line.chars() {
            heightmap[current_vec_index].push(val.to_digit(RADIX).unwrap());
        }
        heightmap[current_vec_index].push(9);
    }

    heightmap.push(Vec::<u32>::new());
    let current_vec_index = (heightmap.len() - 1) as usize;
    for _ in 0..width + 2 {
        heightmap[current_vec_index].push(9);
    }

    let mut basin_sizes = Vec::<u32>::new();

    // Find local mins and their basins
    for i in 1..=height {
        for j in 1..=width {
            if heightmap[i][j] < heightmap[i+1][j] && heightmap[i][j] < heightmap[i-1][j] &&
               heightmap[i][j] < heightmap[i][j+1] && heightmap[i][j] < heightmap[i][j-1] {
                // Find size of this basin, replace vals with 9 to eliminate them
                basin_sizes.push(get_basin_size(i, j, &mut heightmap));
            }
        }
    }

    basin_sizes.sort();
    basin_sizes.reverse();

    let mut basin_multiplier = 1;

    for i in 0..3 {
        println!("Basin {}: {}", i, basin_sizes[i]);
        basin_multiplier *= basin_sizes[i];
    }

    println!("Basin multiplier: {}", basin_multiplier);

}
