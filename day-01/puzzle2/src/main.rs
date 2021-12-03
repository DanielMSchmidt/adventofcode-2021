use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn combined_depth(vector: &Vec<i32>) -> i32 {
    let mut depth = 0;
    for i in vector {
        depth += i;
    }
    depth
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut times_increased = 0;
        // -1 means not available since there is no previous line
        let mut previous_window_depth = -1;

        let mut current_window = Vec::new();

        for line in lines {
            if let Ok(depth_str) = line {
                let depth = depth_str.parse::<i32>().unwrap();
                current_window.push(depth);

                if current_window.len() > 3 {
                    // remove the first item from the vector
                    current_window.remove(0);
                }

                let window_depth = combined_depth(&current_window);

                if previous_window_depth != -1 && previous_window_depth < window_depth {
                    times_increased += 1;
                    println!(
                        "Increasing from {} to {}",
                        previous_window_depth, window_depth
                    );
                }
                if current_window.len() == 3 {
                    previous_window_depth = window_depth;
                }
            }
        }

        println!("Times increased: {}", times_increased);
    } else {
        println!("Could not read file");
    }
}
