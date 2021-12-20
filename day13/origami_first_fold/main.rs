use std::fs;

#[derive(Copy, Clone, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
struct Fold {
    is_horizontal: bool,
    value: i32,
}

fn main() {
    let input_contents = fs::read_to_string("files/origami_input")
        .expect("Unable to read from input");

    let lines = input_contents.lines().collect::<Vec<&str>>();

    let mut points = Vec::<Point>::new();
    let mut folds = Vec::<Fold>::new();

    let mut point_grab = true;

    // Extract points and folds
    for line in lines {
        if line == "" {
            point_grab = false;
        }
        else if point_grab {
            let point_vec = line.split(',').collect::<Vec<&str>>();

            points.push(Point {
                x: point_vec[0].parse::<i32>().unwrap(),
                y: point_vec[1].parse::<i32>().unwrap(),
            });
        }
        else {
            let fold_str_vec = line.split_whitespace().collect::<Vec<&str>>();

            if fold_str_vec[0] != "fold" || fold_str_vec[1] != "along" {
                println!("Error line \"{}\" is not a valid fold string", line);
            }
            else {
                let fold_vec = fold_str_vec[2].split('=').collect::<Vec<&str>>();
                folds.push(Fold {
                    is_horizontal: fold_vec[0] == "y",
                    value: fold_vec[1].parse::<i32>().unwrap(),
                });
            }
        }
    }

    // Run through first fold only
    for fold in folds {
        // Fold
        for point_index in 0..points.len() {
            if fold.is_horizontal {
                if points[point_index].y > fold.value {
                    let fold_offset = 2 * (points[point_index].y - fold.value);
                    points[point_index].y -= fold_offset;
                }
            }
            else {
                if points[point_index].x > fold.value {
                    let fold_offset = 2 * (points[point_index].x - fold.value);
                    points[point_index].x -= fold_offset;
                }
            }
        }

        // Find and eliminate duplicates
        for i in (0..points.len()).rev() {
            for j in 0..i {
                if points[i] == points[j] {
                    points.remove(i);
                }
            }
        }

        break;
    }

    println!("Number of points: {}", points.len());
}
