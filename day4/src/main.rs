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

fn part_one(grid: &Vec<Vec<char>>) -> i32 {
    let width_bound: i32 = (grid.get(0).expect("No items in the vec").len() - 1).try_into().expect("Couldn't get width");
    let height_bound: i32 = (grid.len() - 1).try_into().expect("Couldn't get height");
    let mut forklift_able: i32 = 0;
    
    for (y, line) in (0..).zip(grid.iter()) {    
        for (x, c) in (0..).zip(line.iter()) {
            if !c.eq(&'@') {
                continue;
            }

            let mut count_of_ats = 0;
            for direction in DIRECTIONS {
                let new_pos = (x + direction.0, y + direction.1);
                if in_bounds(new_pos, width_bound, height_bound) {
                    let compare = grid[new_pos.1 as usize][new_pos.0 as usize];

                    if compare == '@' {
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
                forklift_able += 1;
            }
        }
    }
    return forklift_able;
}

fn part_two(grid: &Vec<Vec<char>>) -> i32 {
    return 0;
}

fn in_bounds(next_pos: (i32, i32), width_bound: i32, height_bound: i32) -> bool {
    let width_bound: bool = match next_pos.0 {
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
    
    let height_bound: bool = match next_pos.1 {
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

    return height_bound && width_bound;

}
fn main() {
    let path = "src/1.input";
    let file_buf: String = std::fs::read_to_string(path).expect("Could not read file.");

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in file_buf.lines() {
        grid.push(line.chars().collect());
    }

    let first_result = part_one(&grid);
    let second_result = part_two(&grid);

    println!("{}", first_result);
    println!("{}", second_result);
}
