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
    
    for jolteon in jolteons {
        let mut stack: Vec<String> = Vec::new();
        let max_length = 12;
        
        let mut idx = 0;
        let mut cur = 12;
        
        while stack.len() != max_length {
            stack.push("0".to_string());
            let mut cur_idx = idx;
            let mut final_idx = 0;
            
            let char_vec: Vec<char> = jolteon.chars().collect();
            while cur <= char_vec.len() - cur_idx {
                let head: String = stack.pop().unwrap();
                let head_num: u64 = head.parse().unwrap();
                let cur_str: String = char_vec[cur_idx].to_string();
                let cur_num: u64 = cur_str.parse().unwrap();
                
                if head_num < cur_num {
                    stack.push(cur_str);
                    final_idx = cur_idx;
                } else {
                    stack.push(head)
                }
                cur_idx += 1;
            }
            cur -= 1;
            idx = final_idx + 1;
        }
        let result: u64 = stack.join("").parse().unwrap();
        total += result;
    }
    return total;
}
fn main() {
    let path = "src/1.input";
    let file_buf = std::fs::read_to_string(path).expect("Could not read file");

    let mut jolteons: Vec<&str> = Vec::new();

    for line in file_buf.lines() {
        jolteons.push(line);
    }

    let start_time = Instant::now();

    let first_result = part_one(&jolteons);
    let second_result = part_two(&jolteons);
        
    let end_time = Instant::now();
    let elapsed_total = end_time.duration_since(start_time);

    println!("Total time for results {:?}", elapsed_total);

    println!("{}", first_result);
    println!("{}", second_result)
    
}
