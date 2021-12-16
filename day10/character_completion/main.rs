use std::fs;

fn get_autocomplete_syntax_score(bad_char: char) -> u64 {
    match bad_char {
        ')' => return 1,
        ']' => return 2,
        '}' => return 3,
        '>' => return 4,
        _ => return 0,
    }
}

fn get_matching_char(open_char: char) -> char {
    match open_char {
        '(' => return ')',
        '[' => return ']',
        '{' => return '}',
        '<' => return '>',
        _ => return ':',
    }
}

fn does_char_match(open_char: char, close_char: char) -> bool {
    return close_char == get_matching_char(open_char);
}

fn is_opening(input_char: char) -> bool {
    match input_char {
        '(' | '[' | '{' | '<' => return true,
        _ => return false
    }
}

fn main() {
    let input_contents = fs::read_to_string("files/syntax_chunk_input")
        .expect("Unable to read from input");

    let lines = input_contents.lines().collect::<Vec<&str>>();

    let mut syntax_scores = Vec::<u64>::new();

    for line in lines {
        // Create a stack for opening chars and add a fail char to the bottom
        let mut chunk_stack = Vec::<char>::new();
        chunk_stack.push(':');

        let mut is_line_valid = true;

        for next_char in line.chars() {
            if is_opening(next_char) {
                chunk_stack.push(next_char);
            }
            else {
                let stack_index = chunk_stack.len() - 1;
                if does_char_match(chunk_stack[stack_index], next_char) {
                    chunk_stack.pop();
                }
                else {
                    // Bad line, ignore and move on
                    is_line_valid = false;
                    break;
                }
            }
        }

        // Get autocomplete score for the line
        if is_line_valid {
            let mut current_score: u64 = 0;

            // flip stack and remove invalid character :
            chunk_stack.reverse();
            chunk_stack.pop();

            for open_char in chunk_stack {
                current_score *= 5;
                current_score += get_autocomplete_syntax_score(get_matching_char(open_char));
            }

            syntax_scores.push(current_score);
        }
    }

    // Find middle syntax score
    syntax_scores.sort();
    let middle_score_index = syntax_scores.len() / 2;
    println!("Middle syntax score: {}", syntax_scores[middle_score_index]);
}
