use std::fs;

#[derive(Copy, Clone)]
struct BingoSlot {
    val: u32,
    marked: bool,
}

#[derive(Clone)]
struct BingoCard {
    card: [[BingoSlot; 5]; 5],
    is_winner: bool,
}

fn create_bingo_card_from_5x5_str(vec_5x5: &Vec<&str>) -> BingoCard {
    let mut new_card = BingoCard {
        card: [
            [
                BingoSlot {
                    val: 0,
                    marked: false,
                };
                5
            ];
            5
        ],
        is_winner: false,
    };

    for i in 0..5 {
        let line_str = vec_5x5[i].split_whitespace().collect::<Vec<&str>>();

        let mut j = 0;
        for slot_str in line_str {
            new_card.card[i][j].val = slot_str.parse::<u32>().unwrap();

            j += 1;
            if j >= 5 {
                break;
            }
        }
    }

    return new_card;
}

fn print_bingo_card(bingo_card: &BingoCard) {
    for line in bingo_card.card {
        for slot in line {
            if slot.marked {
                print!("[{}]\t", slot.val);
            }
            else {
                print!("{}\t", slot.val);
            }
        }
        println!("");
    }
}

fn add_call(call_num: u32, bingo_card: &mut BingoCard) {
    for i in 0..5 {
        for j in 0..5 {
            if bingo_card.card[i][j].val == call_num {
                bingo_card.card[i][j].marked = true;
            }
        }
    }
}

fn check_if_winner(bingo_card: &BingoCard) -> bool {
    // Check horizontal
    for i in 0..5 {
        let mut is_winner = true;

        for j in 0..5 {
            if !bingo_card.card[i][j].marked {
                is_winner = false;
                break;
            }
        }

        if is_winner {
            return true;
        }
    }

    // Check vertical
    for j in 0..5 {
        let mut is_winner = true;

        for i in 0..5 {
            if !bingo_card.card[i][j].marked {
                is_winner = false;
                break;
            }
        }

        if is_winner {
            return true;
        }
    }

    // Check downward diag
    let mut is_winner = true;

    for i in 0..5 {
        if !bingo_card.card[i][i].marked {
            is_winner = false;
            break;
        }
    }

    if is_winner {
        return true;
    }

    // Check upward diag
    is_winner = true;

    for i in 0..5 {
        if !bingo_card.card[4-i][i].marked {
            is_winner = false;
            break;
        }
    }

    return is_winner;
}

fn calculate_score(bingo_card: &BingoCard, last_call: u32) -> u32 {
    let mut unmarked_sum = 0;

    for i in 0..5 {
        for j in 0..5 {
            if !bingo_card.card[i][j].marked {
                unmarked_sum += bingo_card.card[i][j].val;
            }
        }
    }

    return unmarked_sum * last_call;
}

fn main() {
    let input_contents = fs::read_to_string("files/bingo_input")
        .expect("Unable to read from input");

    let mut lines = input_contents.lines().collect::<Vec<&str>>();
    let calls_str_vec = lines[0].split(",").collect::<Vec<&str>>();

    let mut calls_vec = Vec::<u32>::new();

    for call in calls_str_vec {
        calls_vec.push(call.parse::<u32>().unwrap());
    }

    lines.remove(0);
    lines.remove(0);

    // Create bingo cards
    let mut cards = Vec::<BingoCard>::new();
    while lines.len() >= 5 {
        cards.push(create_bingo_card_from_5x5_str(&lines));

        for _ in 0..6 {
            lines.remove(0);

            if lines.len() == 0 {
                break;
            }
        }
    }

    // Run through calls and mark cards as they get bingo
    let mut win_counter = 0;
    for call in calls_vec {
        for i in 0..cards.len() {
            if !cards[i].is_winner {
                add_call(call, &mut cards[i]);

                if check_if_winner(&cards[i]) {
                    cards[i].is_winner = true;
                    win_counter += 1;

                    if win_counter == cards.len() {
                        print_bingo_card(&cards[i]);
                        println!("Final Score: {}", calculate_score(&cards[i], call));
                        return;
                    }
                }
            }
        }
    }
}
