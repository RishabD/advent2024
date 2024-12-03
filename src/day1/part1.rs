use crate::day1::utils;

pub fn run() {
    let (mut left, mut right) = utils::read_input();
    left.sort();
    right.sort();
    let mut dist: i32 = 0;
    for (left_num, right_num) in left.iter().zip(right.iter()) {
        dist += (left_num - right_num).abs();
    }
    println!("The distance is {}", dist);
}
