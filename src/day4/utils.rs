use std::{
    fs::File,
    io::{self, BufRead},
};

pub fn read_input() -> Vec<Vec<char>> {
    let reader = io::BufReader::new(File::open("input.txt").unwrap());
    return reader
        .lines()
        .map(|result| result.unwrap().chars().collect::<Vec<char>>())
        .collect();
}
