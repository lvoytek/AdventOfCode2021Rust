use std::fs;

#[derive(Clone, Copy, Eq)]
struct Pos {
    x: usize,
    y: usize,
    cumulative_score: u32,
    heuristic_distance: u32,
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

// Get an estimated distance to the end
fn get_heuristic_distance(x: usize, y: usize, risk_map: &Vec<Vec<u32>>) -> u32 {
    let final_x = risk_map[0].len() - 1;
    let final_y = risk_map.len() - 1;

    // Find basic manhattan distance to get heuristic
    let x_remaining = (final_x - x) as u32;
    let y_remaining = (final_y - y) as u32;

    return x_remaining + y_remaining;
}

fn get_lowest_risk(risk_map: &Vec<Vec<u32>>) -> u32 {
    let width = risk_map[0].len();
    let height = risk_map.len();

    let mut open = Vec::<Pos>::new();
    let mut closed = Vec::<Pos>::new();

    // Start with start node
    open.push(Pos {
        x: 0,
        y: 0,
        cumulative_score: 0,
        heuristic_distance: 0,
    });

    while open.len() > 0 {
        // Find open position with lowest score
        let mut current_pos_index : usize = 0;

        for open_pos_index in 0..open.len() {
            if open[open_pos_index].cumulative_score + open[open_pos_index].heuristic_distance <
                open[current_pos_index].cumulative_score + open[current_pos_index].heuristic_distance {
                current_pos_index = open_pos_index;
            }
        }

        let current_pos = open[current_pos_index].clone();

        // This is the last node, return its cumulative risk
        if current_pos.x == width - 1 && current_pos.y == height - 1 {
            return current_pos.cumulative_score;
        }

        // Swap current pos from open to closed list
        open.remove(current_pos_index);
        closed.push(current_pos);

        // Find neighbors of lowest risk position
        let mut neighbors = Vec::<Pos>::new();

        if current_pos.x > 0 {
            neighbors.push(Pos {
                x: current_pos.x - 1,
                y: current_pos.y,
                cumulative_score: current_pos.cumulative_score + risk_map[current_pos.y][current_pos.x - 1],
                heuristic_distance: get_heuristic_distance(current_pos.x - 1, current_pos.y, risk_map),
            })
        }

        if current_pos.y > 0 {
            neighbors.push(Pos {
                x: current_pos.x,
                y: current_pos.y - 1,
                cumulative_score: current_pos.cumulative_score + risk_map[current_pos.y - 1][current_pos.x],
                heuristic_distance: get_heuristic_distance(current_pos.x, current_pos.y - 1, risk_map),
            })
        }

        if current_pos.x < width - 1 {
            neighbors.push(Pos {
                x: current_pos.x + 1,
                y: current_pos.y,
                cumulative_score: current_pos.cumulative_score + risk_map[current_pos.y][current_pos.x + 1],
                heuristic_distance: get_heuristic_distance(current_pos.x + 1, current_pos.y, risk_map),
            })
        }

        if current_pos.y < height - 1 {
            neighbors.push(Pos {
                x: current_pos.x,
                y: current_pos.y + 1,
                cumulative_score: current_pos.cumulative_score + risk_map[current_pos.y + 1][current_pos.x],
                heuristic_distance: get_heuristic_distance(current_pos.x, current_pos.y + 1, risk_map),
            })
        }

        // For each neighbor:
        //   Ignore if in closed list
        //   Add to open list if not already
        //   Otherwise compare costs and take lower for open list
        for neighbor in neighbors {
            let mut is_on_closed_list = false;
            for i in 0..closed.len() {
                if neighbor == closed[i] {
                    is_on_closed_list = true;
                    break;
                }
            }

            if !is_on_closed_list {
                let mut is_on_open_list = false;

                for i in 0..open.len() {
                    if neighbor == open[i] {
                        is_on_open_list = true;

                        // Compare costs
                        if neighbor.cumulative_score < open[i].cumulative_score {
                            open[i].cumulative_score = neighbor.cumulative_score;
                        }

                        break;
                    }
                }

                if !is_on_open_list {
                    open.push(neighbor.clone());
                }
            }
        }
    }

    return u32::MAX;
}

fn main() {
    let input_contents = fs::read_to_string("files/chiton_risk_input")
        .expect("Unable to read from input");

    let lines = input_contents.lines().collect::<Vec<&str>>();

    let mut risk_map = Vec::<Vec::<u32>>::new();

    // Build numeric matrix with max val outline
    const RADIX: u32 = 10;

    for line in lines {
        risk_map.push(Vec::<u32>::new());
        let current_vec_index = (risk_map.len() - 1) as usize;

        for val in line.chars() {
            risk_map[current_vec_index].push(val.to_digit(RADIX).unwrap());
        }
    }

    // Increase map size to 5x with +1 risk values going down and right (capped at 9)
    // Start with top row of chunks
    let risk_map_start_width = risk_map[0].len();
    let risk_map_start_height = risk_map.len();

    for i in 1..=4 {
        for y in 0..risk_map_start_height {
            for x in 0..risk_map_start_width {
                let mut num_to_add = risk_map[y][x] + i;

                if num_to_add > 9 {
                    num_to_add -= 9;
                }

                risk_map[y].push(num_to_add);
            }
        }
    }

    // Add remaining chunks based on top row
    let risk_map_new_width = risk_map[0].len();
    for i in 1..=4 {
        for y in 0..risk_map_start_height {
            risk_map.push(Vec::<u32>::new());
            let current_vec_index = (risk_map.len() - 1) as usize;

            for x in 0..risk_map_new_width {
                let mut num_to_add = risk_map[y][x] + i;

                if num_to_add > 9 {
                    num_to_add -= 9;
                }

                risk_map[current_vec_index].push(num_to_add);
            }
        }
    }

    // Run A* to find path with lowest risk
    println!("Lowest Risk: {}", get_lowest_risk(&risk_map));
}
