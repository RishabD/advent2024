use std::fs::File;
use std::io::{self, BufRead};

pub fn read_input() -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let reader = io::BufReader::new(File::open("input.txt").unwrap());
    reader.lines().for_each(|line| {
        let content = line.unwrap();
        let mut parts = content.split_whitespace();
        let (left_num, right_num) = (
            parts.next().unwrap().parse::<i32>().unwrap(),
            parts.next().unwrap().parse::<i32>().unwrap(),
        );
        left.push(left_num);
        right.push(right_num);
    });
    return (left, right);
}
