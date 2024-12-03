use std::fs::File;
use std::io::{self, BufRead};

pub fn read_input() -> Vec<Vec<i32>> {
    let mut result = Vec::new();

    let file = File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let content = line.unwrap();
        let mut parts = content.split_whitespace();
        result.push(
            parts
                .map(|string_num| string_num.parse::<i32>().unwrap())
                .collect(),
        )
    }
    return result;
}
