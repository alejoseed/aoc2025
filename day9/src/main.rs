use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Coordinate = (usize, usize);

fn part_one(coordinates: &[Coordinate]) -> i64 {
    let mut greatest_area = 0;
    for (i, coor_a) in coordinates.iter().enumerate() {
        for (j, coor_b) in coordinates.iter().enumerate() {
            if j == i {
                continue;
            }

            let new_area = get_area(coor_a, coor_b);
            greatest_area = std::cmp::max(greatest_area, new_area);
        }
    }
    return greatest_area;
}

fn print_perimeter(original_coords: &[Coordinate]) {    
    let mut w = 0;
    let mut h = 0;

    for coor in original_coords {
        w = std::cmp::max(w, coor.0);
        h = std::cmp::max(h, coor.1);
    }

    let mut board: Vec<Vec<char>> = Vec::new();

    for _y in 0..h + 2 {
        let mut temp: Vec<char> = Vec::new();

        for _x in 0..w + 3 {
            temp.push('.');
        }
        board.push(temp);
    }

    for coor in original_coords {
        board[coor.1][coor.0] = '#'
    };

    for line in board {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
}

fn part_two(coordinates: &[Coordinate]) -> i64 {
    let mut greatest_area = 0;
    let mut green_spots: HashSet<Coordinate> = HashSet::new();

    for (i, coor_a) in coordinates.iter().enumerate() {
        for (j, coor_b) in coordinates.iter().enumerate() {
            if j == i {
                continue;
            }
            if green_spots.contains(coor_a) || green_spots.contains(coor_b) {
                continue;
            }
            
            if coor_a.1 == coor_b.1 {
                for x in coor_a.0 + 1..coor_b.0 {
                    green_spots.insert((x, coor_a.1));
                }
            }
            if coor_a.0 == coor_b.0 {
                for x in coor_a.1 + 1..coor_b.1 {
                    green_spots.insert((coor_a.0, x));
                }
            }
        }
    }

    let mut temp = coordinates.to_vec();

    for green_spot in green_spots.clone() {
        temp.push(green_spot);
    }
    print_perimeter(&temp);

    println!("Backfilling first green");

    // for (i, coor_a) in temp.iter().enumerate() {
    //     if i > 245 {
    //         println!("{} out of {}", i, temp.len());
    //     }
        
    //     for (j, coor_b) in temp.iter().enumerate() {
    //         if j == i {
    //             continue;
    //         }
            
    //         if coor_a.1 == coor_b.1 {
    //             for x in coor_a.0 + 1..coor_b.0 {
    //                 green_spots.insert((x, coor_a.1));
    //             }
    //         }
    //         if coor_a.0 == coor_b.0 {
    //             for x in coor_a.1 + 1..coor_b.1 {
    //                 green_spots.insert((coor_a.0, x));
    //             }
    //         }
    //     }
    // }

    // for green_spot in green_spots.clone() {
    //     temp.push(green_spot);
    // }
    println!("Done Backfilling first green");

    // let res = part_one(&temp);

    return 0;
}

fn main() {
    let path = "/home/alejoseed/aoc2025/day9/src/1.input";
    let lines_result = read_lines(path);

    let lines = match lines_result {
        Ok(l) => l.map_while(Result::ok),
        Err(e) => {
            eprintln!("Error reading file at {}: {}", path, e);
            return;
        }
    };
    let mut points: Vec<Coordinate> = Vec::new();

    for line in lines {
        let point: Vec<&str> = line.split(',').collect();
        let y = point.get(0).unwrap().parse::<usize>().unwrap();
        let x = point.get(1).unwrap().parse::<usize>().unwrap();
        
        points.push((y, x));
    }

    let first_result = part_one(&points);
    println!("{}", first_result);
    let second_result = part_two(&points);

    println!("{}", second_result);
    return;
}

fn get_area(coor_a: &Coordinate, coor_b: &Coordinate) -> i64 {
    let w = coor_a.1.abs_diff(coor_b.1) as i64 + 1;
    let h = coor_a.0.abs_diff(coor_b.0) as i64 + 1;
    let area = w * h;

    return area;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P : AsRef<Path> {
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines()); 
}