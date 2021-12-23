use std::fs;
use std::cmp::Ordering;

#[derive(Clone, Eq, PartialEq)]
struct ElementPair {
    front: char,
    back: char,
}

#[derive(Clone)]
struct Transfer {
    pair: ElementPair,
    insert: char,
}

#[derive(Clone)]
struct ElementPairAmount {
    pair: ElementPair,
    amount: u128,
}

#[derive(Clone, Eq)]
struct ElementAmount {
    element: char,
    amount: u128,
}

impl PartialOrd for ElementAmount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ElementAmount {
    fn cmp(&self, other: &Self) -> Ordering {
        self.amount.cmp(&other.amount)
    }
}

impl PartialEq for ElementAmount {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount
    }
}

fn add_element_pair(new_pair: ElementPair, element_pair_counter: &mut Vec<ElementPairAmount>, amount: u128) {
    let mut pair_already_exists = false;

    for epair_index in 0..element_pair_counter.len() {
        if element_pair_counter[epair_index].pair == new_pair {
            element_pair_counter[epair_index].amount += amount;
            pair_already_exists = true;
            break;
        }
    }

    if !pair_already_exists {
        element_pair_counter.push(ElementPairAmount {
            pair: new_pair,
            amount: amount,
        });
    }
}

fn main() {
    let input_contents = fs::read_to_string("files/polymer_input")
        .expect("Unable to read from input");

    let mut lines = input_contents.lines().collect::<Vec<&str>>();

    let elements : String = lines[0].to_string();

    lines.remove(0);
    lines.remove(0);

    let mut transfers = Vec::<Transfer>::new();

    for line in lines {
        let from_insert = line.split(" -> ").collect::<Vec<&str>>();

        transfers.push(Transfer {
            pair: ElementPair {
                front: from_insert[0].chars().nth(0).unwrap(),
                back: from_insert[0].chars().nth(1).unwrap(),
            },

            insert: from_insert[1].chars().nth(0).unwrap(),
        });
    }

    let mut element_pair_counter = Vec::<ElementPairAmount>::new();

    // Add initial element pairs
    for i in 0..elements.len() - 1 {
        let new_pair = ElementPair {
            front: elements.chars().nth(i).unwrap(),
            back: elements.chars().nth(i+1).unwrap(),
        };

        add_element_pair(new_pair, &mut element_pair_counter, 1);
    }

    // Run through 40 steps
    for _ in 0..40 {
        let mut new_pair_vec = Vec::<ElementPairAmount>::new();

        for pair_count in &element_pair_counter {
            for transfer in &transfers {
                if pair_count.pair == transfer.pair {
                    let front_pair = ElementPair {
                        front: transfer.pair.front,
                        back: transfer.insert,
                    };

                    let back_pair = ElementPair {
                        front: transfer.insert,
                        back: transfer.pair.back,
                    };

                    add_element_pair(front_pair, &mut new_pair_vec, pair_count.amount);
                    add_element_pair(back_pair, &mut new_pair_vec, pair_count.amount);
                    break;
                }
            }
        }

        element_pair_counter = new_pair_vec;
    }

    // find min and max count elements
    let mut count_buckets = Vec::<ElementAmount>::new();

    // Add elements by counting back of all pairs
    for element_pair_count in element_pair_counter {
        let element = element_pair_count.pair.back;
        let mut element_found = false;

        for i in 0..count_buckets.len() {
            if element == count_buckets[i].element {
                count_buckets[i].amount += element_pair_count.amount;
                element_found = true;
            }
        }

        if !element_found {
            count_buckets.push(ElementAmount {
                element: element,
                amount: element_pair_count.amount,
            });
        }
    }

    // Add 1 to first element to account for it
    let first_element = elements.chars().nth(0).unwrap();
    let mut element_found = false;

    for i in 0..count_buckets.len() {
        if first_element == count_buckets[i].element {
            count_buckets[i].amount += 1;
            element_found = true;
        }
    }

    if !element_found {
        count_buckets.push(ElementAmount {
            element: first_element,
            amount: 1,
        });
    }

    count_buckets.sort();
    let cb_last_index = count_buckets.len() - 1;
    println!("{} ({}) - {} ({}) = {}", count_buckets[cb_last_index].element, count_buckets[cb_last_index].amount, count_buckets[0].element, count_buckets[0].amount, count_buckets[cb_last_index].amount - count_buckets[0].amount);
}
