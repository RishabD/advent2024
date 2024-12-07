use crate::day1::utils;

use std::collections::HashMap;

pub fn run() {
    let (left, right) = utils::read_input();
    let right_counter = count_numbers(&right);

    let total: i32 = left
        .iter()
        .map(|left_num| left_num * right_counter.get(left_num).unwrap_or(&0))
        .sum();
    println!("Total: {}", total);
}

fn count_numbers(numbers: &Vec<i32>) -> HashMap<i32, i32> {
    let mut counter = HashMap::new();
    numbers.iter().for_each(|num| {
        *counter.entry(*num).or_insert(0) += 1;
    });
    return counter;
}
