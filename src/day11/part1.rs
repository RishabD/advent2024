use std::collections::HashMap;

use crate::day11::utils;

pub fn run() {
    let stones = utils::read_input();
    let mut cache = HashMap::new();
    let total = stones
        .iter()
        .map(|&stone| utils::get_final_stone_count_from_stone(stone, 0, 25, &mut cache))
        .sum::<u64>();

    println!("Total: {}", total);
}
