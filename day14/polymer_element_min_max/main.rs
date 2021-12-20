use std::fs;
use std::cmp::Ordering;

#[derive(Clone)]
struct Transfer<'a> {
    input: &'a str,
    insert: char,
}

#[derive(Clone, Eq)]
struct ElementAmount {
    element: char,
    amount: u32,
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

fn main() {
    let input_contents = fs::read_to_string("files/polymer_input")
        .expect("Unable to read from input");

    let mut lines = input_contents.lines().collect::<Vec<&str>>();

    let mut elements : String = lines[0].to_string();

    lines.remove(0);
    lines.remove(0);

    let mut transfers = Vec::<Transfer>::new();

    for line in lines {
        let from_insert = line.split(" -> ").collect::<Vec<&str>>();

        transfers.push(Transfer {
            input: from_insert[0],
            insert: from_insert[1].chars().nth(0).unwrap(),
        });
    }

    // Run through 10 steps
    for _ in 0..10 {
        let mut i: usize = 0;
        for _ in 0..elements.len() - 1 {
            for transfer in &transfers {
                if &elements[i..=i+1] == transfer.input {
                    elements.insert(i+1, transfer.insert);
                    i += 1;
                    break;
                }
            }
            i += 1;
        }
    }

    // find min and max count elements
    let mut count_buckets = Vec::<ElementAmount>::new();

    for element in elements.chars() {
        let mut element_found = false;
        for i in 0..count_buckets.len() {
            if element == count_buckets[i].element {
                count_buckets[i].amount += 1;
                element_found = true;
            }
        }

        if !element_found {
            count_buckets.push(ElementAmount {
                element: element,
                amount: 1,
            });
        }
    }

    count_buckets.sort();
    let cb_last_index = count_buckets.len() - 1;
    println!("{} ({}) - {} ({}) = {}", count_buckets[cb_last_index].element, count_buckets[cb_last_index].amount, count_buckets[0].element, count_buckets[0].amount, count_buckets[cb_last_index].amount - count_buckets[0].amount);
}
