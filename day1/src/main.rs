use std::collections::btree_set::Difference;

fn first_part(rotations: &Vec<String>) -> i64 {
    let mut base: i64 = 50;
    let mut num_of_zeros = 0;

    // Initial thing I came up with. Then I thought I'm mostly matching total so
    // match direction {
    //     'L' => {
    //         let total = (base - amount).rem_euclid(100);
    //         if total == 0 {
    //             num_of_zeros += 1;
    //         }
    //         base = total;
    //     }
    //     'R' => {
    //         let total = (base + amount).rem_euclid(100);
    //         if total == 0 {
    //             num_of_zeros += 1;
    //         }
    //         base = total
    //     }
    //     _ => eprintln!("Unexpected direction")
    // };

    for rotation in rotations {
        let direction = rotation.chars().next().unwrap();
        let len = rotation.len();
        let amount: i64 = rotation[1..len].parse().unwrap();
        
        let total = match direction {
            'L' => (base - amount).rem_euclid(100),
            'R' => (base + amount).rem_euclid(100),
            _ => {
                eprintln!("Unexpected direction");
                base
            }
        };
        
        if total == 0 {
            num_of_zeros += 1;
        }
        base = total
    }

    return num_of_zeros
}

fn second_part_correct(rotations: &Vec<String>) -> i64 {
    let mut num_of_zeros = 0;
    let mut base: i64 = 50;
    for rotation in rotations {
        let direction = rotation.chars().next().unwrap();
        let len = rotation.len();
        let mut amount: i64 = rotation[1..len].parse().unwrap();
        
        while amount > 0 {
            base = match direction {
                'L' => (base - 1).rem_euclid(100),
                'R' => (base + 1).rem_euclid(100),
                _ => {println!("Should not be here"); base}
            };

            if base == 0 {
                num_of_zeros += 1;
            }

            amount -= 1;
        }
    }
    return num_of_zeros;
}

fn second_part_math_that_does_not_work_at_all_and_i_dont_know_the_solution(rotations: &Vec<String>) -> i64 {
    let mut base: i64 = 50;
    let mut num_of_zeros = 0;
    let mut extra_rotations = 0;

    for rotation in rotations {
        let direction = rotation.chars().next().unwrap();
        let len = rotation.len();
        let amount: i64 = rotation[1..len].parse().unwrap();
        
        let mut pre_mod_total = match direction {
            'L' => (base - amount),
            'R' => (base + amount),
            _ => {
                eprintln!("Unexpected direction");
                base
           }
        };
        
        let total = pre_mod_total.rem_euclid(100);
        let mut total_iters = 0;

        if base != 0 {
            if pre_mod_total < 0 {
            
                while pre_mod_total < 0 {
                    pre_mod_total += 100;
                    total_iters += 1;
                }
            }
            else if pre_mod_total > 100 {
                while pre_mod_total > 100 {
                    pre_mod_total -= 100;
                    total_iters += 1;
                }
            }
            
        }
                
        if total == 0 {
            num_of_zeros += 1;
        }
        base = total;
        extra_rotations += total_iters;
        println!("{total_iters}")
    }
    return num_of_zeros + extra_rotations;
}



fn main() {
    let path = "/home/alejoseed/Projects/aoc2025/day1/src/1.input";

    let file_buf = std::fs::read_to_string(path).expect("Could not read file");
    let mut rotations: Vec<String> = Vec::new();

    for line in file_buf.lines() {
        rotations.push(line.to_owned().to_string());
    }

    let first_result = first_part(&rotations);
    let second_result = second_part_correct(&rotations);
    println!("First step {first_result}");
    println!("Second step {second_result}");
}
