use std::fs::File;
use std::io::{self, BufRead};

pub fn read_input() -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    let file = File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let content = line.unwrap();
        let mut parts = content.split_whitespace();
        if let (Some(first), Some(second)) = (parts.next(), parts.next()) {
            if let (Ok(num1), Ok(num2)) = (first.parse::<i32>(), second.parse::<i32>()) {
                left.push(num1);
                right.push(num2);
            } else {
                eprintln!("Failed to parse numbers in line: {}", content);
                panic!();
            }
        } else {
            eprintln!("Line doesn't have two parts: {}", content);
            panic!();
        }
    }

    return (left, right);
}
