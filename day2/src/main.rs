use std::time::Instant;
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

fn part_two(ranges: &Vec<(&str, &str)>) -> i64 {
    let mut invalid_ids_sum: i64 = 0;

    for range in ranges {
        let (low, high): (i64, i64) = (range.0.parse().unwrap(), range.1.parse().unwrap());

        for n in low..=high {

            let cur_str: String = n.to_string();
            let str_len = cur_str.len();
            
            let mut pair_size = 1;

            while pair_size != str_len {
                if str_len % pair_size != 0 {
                    pair_size += 1;
                    continue;
                }

                let parts = cur_str.chars().collect::<Vec<char>>();
                let test = parts.chunks(pair_size.try_into().unwrap()).map(|c| c.iter().collect::<String>()).collect::<Vec<String>>();
                
                let first_pair: String = test.first().expect("There should be at least one value").to_owned();
                
                let mut is_matching = true;
                for value in test {
                    if value != first_pair {
                        is_matching = false;
                        break;
                    }
                }
                if is_matching {
                    invalid_ids_sum += n;
                    break;
                }
                pair_size += 1;
            }
        }
    }

    return invalid_ids_sum;
}

fn main() {
    
    let path = "/home/alejoseed/Projects/aoc2025/day2/src/1.input";
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
    
    let start_time = Instant::now();

    let first_result = part_one(&ranges);
    let second_result = part_two(&ranges);
    
    let end_time = Instant::now();
    let elapsed_total = end_time.duration_since(start_time);

    println!("{first_result}");
    println!("{second_result}");
    
    println!("Total time for results {:?}", elapsed_total);
}
