use std::collections::HashSet;

fn part_one(ranges: &Vec<(i64, i64)>, foods: &mut HashSet<i64>) -> i64 {
    let mut fresh_foods = 0;

    for food in foods.clone().iter() {
        for range in ranges {
            if food.ge(&range.0) && food.le(&range.1) {
                // println!("food {} is in range {},{}. Removing from hashset", food, range.0, range.1);
                fresh_foods += 1;
                foods.remove(food);
                break;
            }
        }
    }
    return fresh_foods;
}

fn combine_ranges(ranges: &Vec<(i64, i64)>) -> Vec<(i64,i64)> {
    let mut new_ranges: Vec<(i64, i64)> = Vec::new();
    let mut checked: HashSet<i64> = HashSet::new();

    for i in 0..ranges.len() {
        let cur_range = ranges.get(i).expect("Could not get initial range");
        
        let first_range: i64 = cur_range.0;
        let mut last_range: i64 = cur_range.1; 
        

        for j in 0..ranges.len() {
            if i == j {
                continue;
            }

            let possible_between = ranges.get(j)
                                                        .expect("Could not get between range");

            if last_range.ge(&possible_between.0) && last_range.le(&possible_between.1) {
                last_range = possible_between.1;
            }
        }
        
        match checked.contains(&last_range) {
            true => continue,
            _ => {
                new_ranges.push((first_range, last_range));
                checked.insert(last_range);
            },
        }
    }

    return new_ranges;
}

fn part_two(ranges: &Vec<(i64, i64)>) -> i64 {
    let new_ranges = combine_ranges(ranges);
    println!("The length of new ranges {} and the old was {}", new_ranges.len(), ranges.len());

    let mut possible_foods: i64 = 0;

    for range in new_ranges {
        possible_foods += (range.1 - range.0) + 1
    }

    return possible_foods;
}
fn main() {
    let path = "/home/alejoseed/Projects/aoc2025/day5/src/1.input";
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

    let first_result = part_one(&ranges, &mut foods);

    println!("{}", first_result);
    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let second_result = part_two(&ranges);
    println!("{}", second_result);
}
