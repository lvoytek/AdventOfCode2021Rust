use std::fs;

#[derive(Clone)]
struct Cave<'a> {
    index: usize,
    name: &'a str,
    is_repeatable: bool,
    marked: bool,
    connections: Vec<usize>,
}

// Create a dot file for visualizing the cave system
fn dot_builder(mut caves: Vec<Cave>) {
    let mut output_data: String = "graph caves {\n".to_string();

    // Create nodes
    for cave in &caves {
        output_data.push_str(&cave.index.to_string());
        output_data.push_str(" [label=\"");
        output_data.push_str(cave.name);
        output_data.push_str("\"");

        if cave.is_repeatable {
            output_data.push_str(" shape=box");
        }

        output_data.push_str("];\n");
    }

    // Add edges
    for i in 0..caves.len() as usize {
        caves[i].marked = true;

        for connection in caves[i].connections.clone() {
            if !caves[connection].marked {
                output_data.push_str(&i.to_string());
                output_data.push_str(" -- ");
                output_data.push_str(&connection.to_string());
                output_data.push_str(";\n");
            }
        }
    }

    output_data.push_str("}");

    fs::write("caves.dot", output_data).expect("Unable to write file");
}

// Recursively run through the cave paths, assuming no two adjacent re-traversable caves
// other than one once retraversable small cave
fn get_num_paths_to_end(cave_index: usize, caves: &mut Vec<Cave>, already_retraversed: bool, is_new_retraverse: bool) -> u32 {
    // This is the end node, return 1 for num paths
    if caves[cave_index].name == "end" {
        return 1;
    }

    let mut num_paths = 0;

    // Mark this cave as traversed
    caves[cave_index].marked = true;

    // Recurse into every unmarked / re-traversable cave
    for connection in caves[cave_index].connections.clone() {
        if !caves[connection].marked || caves[connection].is_repeatable {
            num_paths += get_num_paths_to_end(connection, caves, already_retraversed, false);
        }

        if !already_retraversed && !caves[connection].is_repeatable && caves[connection].marked &&
            caves[connection].name != "start" {
            num_paths += get_num_paths_to_end(connection, caves, true, true);
        }
    }

    // Unmark this cave for future traversal if other paths found
    if !is_new_retraverse {
        caves[cave_index].marked = false;
    }

    return num_paths;
}

fn main() {
    let input_contents = fs::read_to_string("files/cave_path_input")
        .expect("Unable to read from input");

    let lines = input_contents.lines().collect::<Vec<&str>>();

    // Build up the graph
    let mut caves = Vec::<Cave>::new();
    let mut start_index: usize = 0;

    for line in lines {
        let line_node_names = line.split("-").collect::<Vec<&str>>();
        let mut node_indecies = Vec::<usize>::new();


        for node in line_node_names {
            let mut already_added = false;
            let mut cave_index : usize = 0;

            for cave in &caves {
                if cave.name == node {
                    cave_index = cave.index;
                    already_added = true;
                }
            }

            if !already_added {
                cave_index = caves.len() as usize;

                let is_lower = node == node.to_lowercase();

                caves.push(Cave {
                    index: cave_index,
                    name: &node,
                    is_repeatable: !is_lower,
                    marked: false,
                    connections: Vec::<usize>::new(),
                });

                // Note the start cave index
                if node == "start" {
                    start_index = cave_index;
                }
            }

            node_indecies.push(cave_index);
        }

        // Add connections
        caves[node_indecies[0]].connections.push(node_indecies[1]);
        caves[node_indecies[1]].connections.push(node_indecies[0]);
    }

    //dot_builder(caves.clone());

    // Starting at start, traverse every path
    let num_paths = get_num_paths_to_end(start_index, &mut caves, false, false);
    println!("Number of paths: {}", num_paths);
}
