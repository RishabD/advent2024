use std::fs::File;
use std::io::{self, BufRead};

use regex::Regex;

pub fn read_input() -> Vec<String> {
    let reader = io::BufReader::new(File::open("input.txt").unwrap());
    return reader.lines().map(|result| result.unwrap()).collect();
}

pub fn perform_multiply(text: &str) -> u32 {
    let number_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let captures = number_regex.captures(text).unwrap();
    let first = &captures[1];
    let second = &captures[2];
    return first.parse::<u32>().unwrap() * second.parse::<u32>().unwrap();
}
