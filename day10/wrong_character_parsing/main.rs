use std::fs;

fn get_bad_syntax_score(bad_char: char) -> u32 {
    match bad_char {
        ')' => return 3,
        ']' => return 57,
        '}' => return 1197,
        '>' => return 25137,
        _ => return 0,
    }
}

fn does_char_match(open_char: char, close_char: char) -> bool {
    match open_char {
        '(' => return close_char == ')',
        '[' => return close_char == ']',
        '{' => return close_char == '}',
        '<' => return close_char == '>',
        _ => return false,
    }
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

    let mut syntax_score = 0;

    for line in lines {
        // Create a stack for opening chars and add a fail char to the bottom
        let mut chunk_stack = Vec::<char>::new();
        chunk_stack.push(':');

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
                    syntax_score += get_bad_syntax_score(next_char);
                    break;
                }
            }
        }
    }

    println!("Overall syntax score: {}", syntax_score);
}
