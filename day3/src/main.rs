use std::time::Instant;

fn part_one(jolteons: &Vec<&str>) -> u32 {
    let mut total: u32 = 0;

    for jolteon in jolteons {
        let mut largest: u32 = 0;
        let bytes = jolteon.as_bytes();

        for i in 0..bytes.len() {
            for j in i+1..bytes.len() {
                if let (Ok(char1), Ok(char2)) = (
                    std::str::from_utf8(&bytes[i..=i]),
                    std::str::from_utf8(&bytes[j..=j])
                ) {
                    let temp: u32 = (char1.to_owned() + char2).parse().unwrap();

                    largest = match temp {
                        _n if largest < temp => temp,
                        _ => largest
                    };
                }
            }
        }
        total += largest;
    }
    
    return total;
}

fn part_two(jolteons: &Vec<&str>) -> u64 {
    let mut total: u64 = 0;
    let mut stack: Vec<&u32> = Vec::new();
    
    for jolteon in jolteons {
        let mut remaining_numbers = 12;
        let bytes = jolteon.as_bytes();
        
        for i in 0..bytes.len() {
            if stack.iter().peekable().peek() < bytes[i].try_into().unwrap() {

            }
        }
    }
    return total;
}
fn main() {
    let path = "/home/alejoseed/aoc2025/day3/src/1.input";
    let file_buf = std::fs::read_to_string(path).expect("Could not read file");

    let mut jolteons: Vec<&str> = Vec::new();

    for line in file_buf.lines() {
        jolteons.push(line);
    }

    let start_time = Instant::now();

    let first_result = part_one(&jolteons);

        
    let end_time = Instant::now();
    let elapsed_total = end_time.duration_since(start_time);

    println!("Total time for results {:?}", elapsed_total);

    println!("{}", first_result)
}
