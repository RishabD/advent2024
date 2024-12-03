use std::fs::File;
use std::io::{self, BufRead};

use regex::Regex;

pub fn read_input() -> Vec<String> {
    let reader = io::BufReader::new(File::open("input.txt").unwrap());
    return reader.lines().map(|result| result.unwrap()).collect();
}

pub fn perform_mult(text: &str) -> i32 {
    let mult_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let captures = mult_re.captures(text).unwrap();
    let first = &captures[1];
    let second = &captures[2];
    return first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap();
}
