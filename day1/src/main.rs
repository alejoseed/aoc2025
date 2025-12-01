fn main() {
    let path = "/home/alejoseed/aoc2025/day1/src/1.input";

    let file_buf = std::fs::read_to_string(path).expect("Could not read file");
    let mut rotations: Vec<String> = Vec::new();

    for line in file_buf.lines() {
        rotations.push(line.to_owned().to_string());
    }

    let mut base: i64 = 50;
    let mut num_of_zeros = 0;

    for rotation in rotations {
        let direction = rotation.chars().next().unwrap();
        let len = rotation.len();
        let amount: i64 = rotation[1..len].parse().unwrap();
        
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
        
        let total = match direction {
            'L' => (base - amount).rem_euclid(100),
            'R' => (base + amount).rem_euclid(100),
            _ => {
                eprintln!("Unexpected direction");
                base
            }
        };

        if total == 0 {
            num_of_zeros += 1
        }
        base = total;
    }

    println!("{num_of_zeros}");
}
