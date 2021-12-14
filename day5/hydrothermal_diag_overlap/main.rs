use std::fs;

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
struct Line {
    p1: Point,
    p2: Point,
}

fn extract_line_from_str(line_str: &str) -> Line {
    let point_vec = line_str.split(" -> ").collect::<Vec<&str>>();

    let p1_vec = point_vec[0].split(',').collect::<Vec<&str>>();
    let p2_vec = point_vec[1].split(',').collect::<Vec<&str>>();

    let p1 = Point {
        x: p1_vec[0].parse::<i32>().unwrap(),
        y: p1_vec[1].parse::<i32>().unwrap(),
    };

    let p2 = Point {
        x: p2_vec[0].parse::<i32>().unwrap(),
        y: p2_vec[1].parse::<i32>().unwrap(),
    };

    return Line {
        p1: p1,
        p2: p2,
    };
}

fn is_horizontal(line: &Line) -> bool {
    return line.p1.y == line.p2.y;
}

fn is_vertical(line: &Line) -> bool {
    return line.p1.x == line.p2.x;
}

fn is_downward_diag(line: &Line) -> bool {
    return (line.p2.x - line.p1.x) == (line.p2.y - line.p1.y);
}

fn is_upward_diag(line: &Line) -> bool {
    return (line.p2.x - line.p1.x) == (line.p1.y - line.p2.y);
}

fn main() {
    let input_contents = fs::read_to_string("files/lines_input")
        .expect("Unable to read from input");

    let line_strs = input_contents.lines().collect::<Vec<&str>>();
    let mut lines = Vec::<Line>::new();

    // Extract horizontal, vertical and diagonal lines
    for line_str in line_strs {
        let new_line = extract_line_from_str(line_str);

        if is_horizontal(&new_line) || is_vertical(&new_line) || is_downward_diag(&new_line) || is_upward_diag(&new_line) {
            lines.push(new_line);
        }
    }

    // Find the largest x and y vals to determine diagram size
    let mut largest_x = 0;
    let mut largest_y = 0;

    for line in &lines {
        if line.p1.x > largest_x {
            largest_x = line.p1.x;
        }

        if line.p1.y > largest_y {
            largest_y = line.p1.y;
        }

        if line.p2.x > largest_x {
            largest_x = line.p2.x;
        }

        if line.p2.y > largest_y {
            largest_y = line.p2.y;
        }
    }

    // Fill out the grid
    let mut vent_grid = vec![vec![0; (largest_y + 1) as usize]; (largest_x + 1) as usize];

    for line in &lines {
        if is_horizontal(line) {
            if line.p2.x > line.p1.x {
                for i in (line.p1.x)..=(line.p2.x) {
                    vent_grid[i as usize][line.p1.y as usize] += 1;
                }
            }
            else {
                for i in (line.p2.x)..=(line.p1.x) {
                    vent_grid[i as usize][line.p1.y as usize] += 1;
                }
            }
        }
        else if is_vertical(line) {
            if line.p2.y > line.p1.y {
                for i in (line.p1.y)..=(line.p2.y) {
                    vent_grid[line.p1.x as usize][i as usize] += 1;
                }
            }
            else {
                for i in (line.p2.y)..=(line.p1.y) {
                    vent_grid[line.p1.x as usize][i as usize] += 1;
                }
            }
        }
        else if is_downward_diag(line) {
            if line.p2.x > line.p1.x {
                for i in 0..=(line.p2.x - line.p1.x) {
                    vent_grid[(i + line.p1.x) as usize][(i + line.p1.y) as usize] += 1;
                }
            }
            else {
                for i in 0..=(line.p1.x - line.p2.x) {
                    vent_grid[(i + line.p2.x) as usize][(i + line.p2.y) as usize] += 1;
                }
            }
        }
        else {
            if line.p2.x > line.p1.x {
                for i in 0..=(line.p2.x - line.p1.x) {
                    vent_grid[(i + line.p1.x) as usize][(line.p1.y - i) as usize] += 1;
                }
            }
            else {
                for i in 0..=(line.p1.x - line.p2.x) {
                    vent_grid[(i + line.p2.x) as usize][(line.p2.y - i) as usize] += 1;
                }
            }
        }
    }

    // Find number of overlapping
    let mut num_overlapping = 0;
    for i in 0..=largest_x {
        for j in 0..=largest_y {
            if vent_grid[i as usize][j as usize] >= 2 {
                num_overlapping += 1;
            }
        }
    }

    println!("Overlapping Points: {}", num_overlapping);
}
