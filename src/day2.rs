use std::{collections::{btree_map::Values, HashSet}, fs};


pub fn solve() {
    let min_max_pairs = parse_file();

    let mut totals: Vec<u64> = vec![];

    for pair in min_max_pairs {
        for value in pair.min..pair.max {
            let value_str = value.to_string();
            if value_str.len() % 2 == 0 {
                let first_half: &str = 
                    &value_str.as_str()[0..value_str.len() / 2];
                let second_half: &str =
                    &value_str.as_str()[value_str.len() / 2..value_str.len()];

                if first_half == second_half {
                    totals.push(value);
                }
            }
        }
    }

    let mut running_total: u64 = 0;
    for number in totals {
        running_total += number;
    }

    println!("{}", running_total);
}



fn parse_file() -> Vec<MinMax> {
    let whole_file = fs::read_to_string("inputday2.txt").unwrap();

    let string_pairs = whole_file.split(',').collect::<Vec<&str>>();

    let mut min_max_pairs: Vec<MinMax> = vec![];

    for pair in string_pairs {
        let split_pair = pair.split('-').collect::<Vec<&str>>();

        min_max_pairs.push(MinMax {
            min: split_pair[0].parse::<u64>().unwrap(),
            max: split_pair[1].parse::<u64>().unwrap(),
        });
    }

    min_max_pairs
}

struct MinMax {
    min: u64,
    max: u64,
}
