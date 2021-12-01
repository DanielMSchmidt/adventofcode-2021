use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut times_increased = 0;
        // -1 means not available since there is no previous line
        let mut previous_depth = -1; 
        for line in lines {
            if let Ok(depth_str) = line {
                let depth = depth_str.parse::<i32>().unwrap();


                if previous_depth != -1 && previous_depth < depth {
                    times_increased += 1;
                    println!("Increasing from {} to {}", previous_depth, depth);
                }
                previous_depth = depth;
            }
        }

        println!("Times increased: {}", times_increased);
    } else {
        println!("Could not read file");
    }
}
