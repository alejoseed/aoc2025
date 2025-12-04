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
        let mut stack: Vec<&str> = Vec::new();
        let max_length = 12;

        
        let mut idx = 0;
        let mut cur = 12;
        
        while stack.len() != max_length {
            stack.push("0");
            let mut cur_idx = idx;
            let mut final_idx = 0;
            
            let char_vec: Vec<char> = jolteon.chars().collect();
            while cur < char_vec.len() - cur_idx{
                let head = stack.pop().unwrap();
                let head_num: u64 = head.parse().unwrap();
                let cur_num: u64 = char_vec[cur_idx].into();

                // if head_num < cur_num {
                //     stack.append(char_vec[cur_idx].to_string().to_owned());
                //     final_idx = cur_idx;
                // } else {
                //     stack.append(head)
                // }
                cur_idx += 1

            }
        }
        let mut start_point = 0;
        let bytes = jolteon.as_bytes();
        
        //iterate any amount of times to find the greatest possible number 
        // with the cabiat that there should be 12 - i available chars to the right of the array.
        while start_point + 1 != 13 {
            
        }

        // There can only be at most 12 entries to the stack
        for n in 0..12 {
            
        }
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

    println!("{}", first_result)
}
