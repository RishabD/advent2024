use crate::day1::utils;

use std::collections::HashMap;

pub fn run() {
    let (left, right) = utils::read_input();
    let mut counter: HashMap<i32, i32> = HashMap::new();

    for right_num in right.iter() {
        *counter.entry(right_num.clone()).or_insert(0) += 1;
    }
    let mut similarity: i32 = 0;
    for left_num in left.iter() {
        similarity += left_num * counter.get(left_num).unwrap_or(&0);
    }
    println!("The distance is {}", similarity);
}
