use std::collections::HashSet;

fn part_one(ranges: &Vec<(i64, i64)>, foods: HashSet<i64>) -> i64 {
    let mut fresh_foods = 0;

    for food in foods {
        for range in ranges {
            if food >= range.0 && food <= range.1 {

            }       
        }
    }
    return fresh_foods;
}

fn main() {
    let path = "/home/alejoseed/aoc2025/day5/src/1.input";
    let buf = std::fs::read_to_string(path).expect("No file found.");

    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let mut foods: HashSet<i64> = HashSet::new();
    
    let mut append_ranges = true;

    for line in buf.lines() {
        if line.is_empty() {
            append_ranges = false;
            continue;
        }

        match append_ranges {
            true => {
                let (a_str, b_str) = line.split_once("-")
                                            .expect("Could not split");
                let (a, b): (i64, i64) = (a_str.parse().unwrap(), b_str.parse().unwrap());

                ranges.push((a, b));
            },
            _ => {
                foods.insert(line.parse().expect("Could not get number"));
            }
        }
    }

    let first_result = part_one(&ranges, &foods);

    println!("{}", first_result);
}
