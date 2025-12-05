use std::collections::HashSet;

const DIRECTIONS: [(i32, i32); 8] = [
    // LEFT, RIGHT
    (-1, 0), (1, 0),
    // UP DOWN
    (0, -1), (0, 1),
    // LEFT UP RIGHT UP
    (-1, -1), (1, -1),
    // LEFT DOWN RIGHT DOWN
    (-1, 1), (1, 1)
];

// fn part_one(grid: &Vec<Vec<char>>) -> i32 {
//     let width_bound: i32 = (grid.get(0).expect("No items in the vec").len() - 1).try_into().expect("Couldn't get width");
//     let height_bound: i32 = (grid.len() - 1).try_into().expect("Couldn't get height");
//     let mut forklift_able: i32 = 0;
    
//     for y in 0..=height_bound {    
//         for x in 0..=width_bound {
//             let c = grid[y as usize][x as usize];
//             if !c.eq(&'@') {
//                 continue;
//             }

//             let mut count_of_ats = 0;
//             for direction in DIRECTIONS {
//                 let new_pos = (x + direction.0, y + direction.1);
//                 if in_bounds(new_pos, width_bound, height_bound) {
//                     let compare = grid[new_pos.1 as usize][new_pos.0 as usize];

//                     if compare == '@' {
//                         if count_of_ats < 4 {
//                             count_of_ats += 1;
//                         }
//                         else {
//                             break;
//                         }
//                     }
//                 }
//             }
            
//             if count_of_ats < 4 {
//                 forklift_able += 1;
//             }
//         }
//     }
//     return forklift_able;
// }

fn part_one_mut(grid: &mut Vec<Vec<char>>) -> i32 {
    let width_bound: i32 = (grid.get(0).expect("No items in the vec").len() - 1).try_into().expect("Couldn't get width");
    let height_bound: i32 = (grid.len() - 1).try_into().expect("Couldn't get height");
    let mut forklift_able: i32 = 0;
    
    let mut replacible_coordinates: HashSet<(i32, i32)> = HashSet::new();
    
    for y in 0..=height_bound {    
        for x in 0..=width_bound {
            let c = grid[y as usize][x as usize];
                    
            if !c.to_ascii_lowercase().eq(&'@') {
                continue;
            }

            let mut count_of_ats = 0;
            for direction in DIRECTIONS {
                let new_pos: (i32, i32) = (y + direction.1, x + direction.0);
                if in_bounds(new_pos, width_bound, height_bound) {
                    let compare = grid[new_pos.0 as usize][new_pos.1 as usize];

                    if compare.to_ascii_lowercase() == '@' {
                        if count_of_ats < 4 {
                            count_of_ats += 1;
                        }
                        else {
                            break;
                        }
                    }
                }
            }
            
            if count_of_ats < 4 {
                replacible_coordinates
                                                .insert((y, x));
                forklift_able += 1;
            }
        }
    }

    for coordinate in replacible_coordinates {
        grid[coordinate.0 as usize][coordinate.1 as usize] = 'x';
    }

    return forklift_able;
}

fn part_two(grid: &mut Vec<Vec<char>>) -> (i32, i32) {
    let mut forklist_able = 0;
    let mut first_result = 0;
    let mut initial_run = true;

    loop {
        let forklist_able_batch = part_one_mut(grid);
        
        if initial_run {
            first_result = forklist_able_batch;
            initial_run = false;
        }

        forklist_able += forklist_able_batch;

        if forklist_able_batch == 0 {
            break;
        } 
    }

    return (first_result, forklist_able);
}

fn in_bounds(next_pos: (i32, i32), width_bound: i32, height_bound: i32) -> bool {    
    let height_bound: bool = match next_pos.0 {
        n if n < 0 => {
            false
        },
        n if n > height_bound => {
            false
        },
        _ => {
            true
        }
    };

    let width_bound: bool = match next_pos.1 {
        n if n < 0 => {
            false
        }
        n if n > width_bound => {
            false
        }
        _ => {
            true
        }
    };

    return height_bound && width_bound;
}
fn main() {
    let path = "src/1.input";
    let file_buf: String = std::fs::read_to_string(path).expect("Could not read file.");

    let mut grid: Vec<Vec<char>> = Vec::new();
    
    for line in file_buf.lines() {
        grid.push(line.chars().collect());
    }
    
    let mut grid_copy = grid.clone();

    let (first_result, second_result) = part_two(&mut grid_copy);

    println!("{}", first_result);
    println!("{}", second_result);
}
