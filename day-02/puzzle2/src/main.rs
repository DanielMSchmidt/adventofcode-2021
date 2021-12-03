use std::fs::File;
use std::io::{self, prelude::*, BufReader};

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl From<String> for Command {
    fn from(s: String) -> Self {
        let vector = s.split(" ").collect::<Vec<&str>>();
        let value = vector[1].parse::<i32>().unwrap();

        match vector[0] {
            "forward" => Command::Forward(value),
            "down" => Command::Down(value),
            "up" => Command::Up(value),
            _ => panic!("Unknown command"),
        }
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let command = Command::from(line);

        match command {
            Command::Forward(value) => {
                horizontal += value;
                depth += aim * value;
            }
            Command::Down(value) => aim += value,
            Command::Up(value) => aim -= value,
        }
    }
    println!(
        "{} forward and {} depth: {}",
        horizontal,
        depth,
        horizontal * depth
    );

    Ok(())
}
