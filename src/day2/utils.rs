use std::fs::File;
use std::io::{self, BufRead};

pub fn read_input() -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let reader = io::BufReader::new(File::open("input.txt").unwrap());
    for line in reader.lines() {
        let content = line.unwrap();
        let parts = content.split_whitespace();
        result.push(
            parts
                .map(|string_num| string_num.parse::<i32>().unwrap())
                .collect(),
        )
    }
    return result;
}

pub fn level_is_safe(level: &Vec<i32>) -> bool {
    return (strict_ascending(level) || strict_descending(level)) && valid_diffs(level);
}

fn strict_ascending(level: &Vec<i32>) -> bool {
    return level
        .iter()
        .zip(level.iter().skip(1))
        .all(|(first, second)| first < second);
}

fn strict_descending(level: &Vec<i32>) -> bool {
    return level
        .iter()
        .zip(level.iter().skip(1))
        .all(|(first, second)| first > second);
}

fn valid_diffs(level: &Vec<i32>) -> bool {
    return level
        .iter()
        .zip(level.iter().skip(1))
        .all(|(first, second)| first != second && (first - second).abs() <= 3);
}
