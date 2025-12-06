use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Div;
use std::path::Path;

fn part_one(numbers: &Vec<Vec<i64>>, operations: &Vec<char>) -> i64 {
    let mut stack = [0i64; 1000];
    
    for (i, operation) in operations.iter().enumerate() {
        let mut cur_stack: Vec<i64> = Vec::new();

        for y in numbers {
            cur_stack.push(y[i]);
        }

        let total = match operation {
            '+' => cur_stack.iter().fold(0, |acc, x| acc + x),
            '*' => cur_stack.iter().fold(1, |acc, x| acc * x),
            _ => {println!("This should not happend ever"); 0}
        };

        stack[i] = total;
    }
    
    
    return stack.iter().sum();
}

fn part_two_incorrect_assumption(numbers: &[Vec<i64>], operations: &[char]) -> i64 {
    let mut stack = [0i64; 1000];
    let ops_vec: Vec<_> = operations.iter().collect();

    for (i, operation) in ops_vec.iter().enumerate().rev() {
        let mut cur_stack: Vec<i64> = Vec::new();
        let mut last_stack: Vec<i64> = Vec::new();

        let mut largest = 0;
        
        for y in numbers.iter().rev() {
            cur_stack.push(y[i]);
            largest = std::cmp::max(largest, y[i]);
        }

        let bases = largest.ilog10() + 1;

        for base in (1..=bases).rev() {
            let local_total: String = match operation {
                    _ => cur_stack.iter().fold("".to_string(), |acc, x| {
                        let d= x.ilog10() + 1;
                        if d < base {
                            return acc
                        }
                        let result = x.div(10_i64.strict_pow(d - base)).rem_euclid(10).to_string();
                        result + &acc 
                    }),
            };
            
            last_stack.push(local_total.parse().unwrap());
        }
    }
    return 0;
}

fn part_two(numbers: &[String], operations: &[char]) -> i64 {
    let mut stack = [0i64; 1000];
    let ops_vec: Vec<_> = operations.iter().collect();
    let mut line_length = (numbers[0].len() - 1) as i64;

    for (i, operation) in ops_vec.iter().enumerate().rev() {
        while line_length >= 0 {
            let mut cur_stack: Vec<char> = Vec::new();

            for line in numbers.iter() {
                let number = line.as_bytes()[line_length as usize] as char;

                let number: char = match number {
                    ' ' => ' ',
                    _ => number
                };

                cur_stack.push(number);
            }

            let should_insert= cur_stack.iter().any(|c| !c.is_whitespace());

            if should_insert {
                let final_num: i64 = cur_stack.iter().collect::<String>().trim().to_string().parse().unwrap();

                match operation {
                    '+' => {
                        stack[i] = stack[i] + final_num;
                    },
                    '*' => {
                        if stack[i] == 0 {
                            stack[i] = 1;
                        }
                        stack[i] = stack[i] * final_num;
                    },
                    _ => {stack[i] = stack[i]}
                };

                cur_stack.clear();
            } else {
                line_length -= 1;
                break;
            }

            line_length -= 1;
        }
    }
    return stack.iter().sum();
}

fn main() {
    let path = "/home/alejoseed/Projects/aoc2025/day6/src/1.input";
    let lines_result = read_lines(path);
    
    let lines: Vec<String> = match lines_result {
        Ok(l) => l.map_while(Result::ok).collect(),
        Err(e) => {
            eprintln!("Error reading file at {}: {}", path, e);
            return;
        }
    };

    let operations: Vec<char> = lines
                                .last()
                                .expect("Error in last line")
                                .chars()
                                .filter(|f| !f.is_whitespace())
                                .collect();

    let line_slice = &lines[0..lines.len() - 1];
    let numbers: Vec<Vec<i64>> = line_slice
        .iter()
        .map(|l| {
            l.split_whitespace()
            .filter_map(|str_digit| {
                str_digit.parse::<i64>().ok()
            }).collect()
        }).collect();
    let sheet_length: usize = operations.len();
    let verify_lengths = numbers.iter().all(|x| x.len() == sheet_length);

    if verify_lengths == false {
        eprintln!("Error parsing lines, operations do not match lines.");
        return;
    }

    let first_result = part_one(&numbers, &operations);
    // let second_result = part_two_incorrect_assumption(&numbers, &operations);

    let second_result = part_two(&lines[0..lines.len() - 1], &operations);
    println!("{}", first_result);
    println!("{}", second_result);

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines());
}
