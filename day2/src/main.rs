use std::collections::HashSet;

fn part_one(ranges: &Vec<(&str, &str)>) -> i64 {
    let mut invalid_ids_sum: i64 = 0;

    for range in ranges {
        let (low, high): (i64, i64) = (range.0.parse().unwrap(), range.1.parse().unwrap());
        let mut cur = low;

        while cur <= high {
            let cur_str: String = cur.to_string();
            if cur_str.len() % 2 == 0 {
                let (first, second) = cur_str.split_at(cur_str.len() / 2);
                if first == second {
                    invalid_ids_sum += cur;
                }
            }
            cur += 1;
        }
    }

    return invalid_ids_sum;
}

// 11-22 still has two invalid IDs, 11 and 22.
// 95-115 now has two invalid IDs, 99 and 111.
// 998-1012 now has two invalid IDs, 999 and 1010.
// 1188511880-1188511890 still has one invalid ID, 1188511885.
// 222220-222224 still has one invalid ID, 222222.
// 1698522-1698528 still contains no invalid IDs.
// 446443-446449 still has one invalid ID, 446446.
// 38593856-38593862 still has one invalid ID, 38593859.
// 565653-565659 now has one invalid ID, 565656.
// 824824821-824824827 now has one invalid ID, 824824824.
// 2121212118-2121212124 now has one invalid ID, 2121212121.

fn part_two(ranges: &Vec<(&str, &str)>) -> i64 {
    let mut invalid_ids_sum: i64 = 0;

    for range in ranges {
        let (low, high): (i64, i64) = (range.0.parse().unwrap(), range.1.parse().unwrap());
        let mut cur = low;

        while cur <= high {
            let cur_str: String = cur.to_string();
            let mut cur_seq: &str = "";
            
            for char in cur_str.chars() {
                
            }
            let mut sequences: HashSet<String> = HashSet::new();

            
        }
    }

    return invalid_ids_sum;
}

fn main() {
    let path = "/home/alejoseed/aoc2025/day2/src/1.input";
    let file_buf = std::fs::read_to_string(path).expect("Could not read file");
    
    let mut ranges: Vec<(&str, &str)> = Vec::new();
    
    for range in file_buf.lines() {
        let parts = range.split(",");
        let collection = parts.collect::<Vec<&str>>();
        
        for entry in collection {
            let (low, high): (&str, &str) = entry.split_once("-").expect("No - found");
            ranges.push((low, high));
        }
    }

    let first_result = part_one(&ranges);
    let second_result = part_two(&ranges);

    // println!("Len of ranges: {}", ranges.len());
    println!("{first_result}");
    println!("{second_result}");

}
