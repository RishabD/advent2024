use crate::day1::utils;

pub fn run() {
    let (mut left, mut right) = utils::read_input();
    left.sort();
    right.sort();
    let total: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(left_num, right_num)| (left_num - right_num).abs())
        .sum();

    println!("Total: {}", total);
}
