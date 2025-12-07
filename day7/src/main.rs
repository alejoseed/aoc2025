use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn part_one(lines: &[String], s_pos: &(usize, usize)) -> u32 {
    let mut total_splits: u32 = 0;
    let mut current_rays: HashSet<(usize, usize)> = HashSet::new();
    current_rays.insert(*s_pos);
    
    for (i, line) in lines.iter().enumerate() {
        if i == lines.len() - 1 {
            break;
        }

        // println!("Is S free to move to next range: {}", i + 1);
        let new_rays = expand_ray(&current_rays, lines);
        current_rays.clear();
        
        total_splits += new_rays.0;
        current_rays = new_rays.1;
    }

    return total_splits;
}


fn expand_ray(rays: &HashSet<(usize, usize)>, lines: &[String]) -> (u32, HashSet<(usize, usize)>) {
    let mut new_rays: HashSet<(usize, usize)> = HashSet::new();
    let mut splits = 0;

    for ray in rays {
        let next_line = lines.get(ray.0 + 1).unwrap();
        let next_spot = *next_line.as_bytes().get(ray.1).unwrap() as char;

        match next_spot {
            '.' => {
                new_rays.insert((ray.0 + 1, ray.1));
            },
            '^' => {
                splits += 1;
                new_rays.insert((ray.0 + 1, ray.1 - 1));
                new_rays.insert((ray.0 + 1, ray.1 + 1));
            },
            _ => {
                println!("Should never happend?");
            }
        }
    }
    return (splits, new_rays);
}

fn expand_ray_two(rays: &HashMap<(usize, usize), u64>, lines: &[String]) -> HashMap<(usize, usize), u64> {
    let mut new_rays: HashMap<(usize, usize), u64> = HashMap::new();

    let mut rays_vec: Vec<(usize, usize)> = rays.iter().map(|(key, _)| *key).collect();
    rays_vec.sort_by(|a,b| {
        a.1.cmp(&b.1)
    });
    // 2 10 11 11 2 1 1 1
    for ray in rays_vec {
        let next_line = lines.get(ray.0 + 1).unwrap();
        let next_spot = *next_line.as_bytes().get(ray.1).unwrap() as char;
        match next_spot {
            '.' => {
                let ray_val = rays.get(&(ray.0, ray.1)).unwrap();
                let spot_val = match new_rays.get(&(ray.0 + 1, ray.1)){
                    Some(val) => *val,
                    _ => 0,
                };
                new_rays.insert((ray.0 + 1, ray.1), *ray_val + spot_val);
            },
            '^' => {
                let ray_val = rays.get(&(ray.0, ray.1)).unwrap();
                let left_val = match new_rays.get(&(ray.0 + 1, ray.1 - 1)) {
                    Some(val) => *val,
                    _ => 0,
                };
                let right_val = match new_rays.get(&(ray.0 + 1, ray.1 + 1)){
                    Some(val) => *val,
                    _ => 0,
                };
                
                new_rays.insert((ray.0 + 1, ray.1 - 1), left_val + ray_val);
                new_rays.insert((ray.0 + 1, ray.1 + 1), right_val + ray_val);
                
                println!("{}, {}", left_val, right_val)
            },
            _ => {
                println!("Should never happend?");
            }
        }
    }
    return new_rays;
}


fn part_two(lines: &[String], s_pos: &(usize, usize)) -> u64 {
    let mut current_rays: HashMap<(usize, usize), u64> = HashMap::new();
    current_rays.insert(*s_pos, 1);
    
    for (i, line) in lines.iter().enumerate() {
        if i == lines.len() - 1 {
            break;
        }

        println!("Is S free to move to next range: {}", i + 1);
        let new_rays = expand_ray_two(&current_rays, lines);
        current_rays.clear();

        current_rays = new_rays;

    }

    return current_rays.values().sum();
}

fn main() {
    let path = "/home/alejoseed/Projects/aoc2025/day7/src/1.input";
    let lines_result = read_lines(path);

    let lines: Vec<String> = match lines_result {
        Ok(l) => l.map_while(Result::ok).collect(),
        Err(e) => {
            eprintln!("Error reading file at {}: {}", path, e);
            return;
        }
    };
    
    let s_pos: (usize, usize) = (0, lines.first().unwrap().find(|s| s == 'S').unwrap());

    let first_result = part_one(&lines, &s_pos);
    let second_result = part_two(&lines, &s_pos);

    println!("{}", first_result);
    println!("{}", second_result);

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path> {
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines());
}