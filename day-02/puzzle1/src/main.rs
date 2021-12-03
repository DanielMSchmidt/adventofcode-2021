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
    let mut forward = 0;
    let mut depth = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let command = Command::from(line);

        match command {
            Command::Forward(value) => forward += value,
            Command::Down(value) => depth += value,
            Command::Up(value) => depth -= value,
        }
    }
    println!(
        "{} forward and {} depth: {}",
        forward,
        depth,
        forward * depth
    );

    Ok(())
}
