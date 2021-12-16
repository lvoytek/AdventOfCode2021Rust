use std::fs;

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
        heightmap[0].push(u32::MAX);
    }

    for line in lines {
        heightmap.push(Vec::<u32>::new());
        let current_vec_index = (heightmap.len() - 1) as usize;

        heightmap[current_vec_index].push(u32::MAX);
        for val in line.chars() {
            heightmap[current_vec_index].push(val.to_digit(RADIX).unwrap());
        }
        heightmap[current_vec_index].push(u32::MAX);
    }

    heightmap.push(Vec::<u32>::new());
    let current_vec_index = (heightmap.len() - 1) as usize;
    for _ in 0..width + 2 {
        heightmap[current_vec_index].push(u32::MAX);
    }



    let mut total_risk = 0;

    // Find local mins
    for i in 1..=height {
        for j in 1..=width {
            if heightmap[i][j] < heightmap[i+1][j] && heightmap[i][j] < heightmap[i-1][j] &&
               heightmap[i][j] < heightmap[i][j+1] && heightmap[i][j] < heightmap[i][j-1] {
                total_risk += heightmap[i][j] + 1;
            }
        }
    }

    println!("Total risk: {}", total_risk);


}
