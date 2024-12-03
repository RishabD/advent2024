use crate::day2::utils;

pub fn run() {
    let levels = utils::read_input();
    let count: i32 = levels
        .iter()
        .map(level_is_safe_variants)
        .map(|is_safe| if is_safe { 1 } else { 0 })
        .sum();
    println!("The number of safe levels is {}", count)
}

// Brute force scales for small number of levels
fn level_is_safe_variants(level: &Vec<i32>) -> bool {
    if utils::level_is_safe(level) {
        return true;
    }
    for i in 0..level.len() {
        let new_level: Vec<i32> = level
            .iter()
            .enumerate()
            .filter(|(index, _)| i != *index)
            .map(|(_, val)| *val)
            .collect();
        if utils::level_is_safe(&new_level) {
            return true;
        }
    }

    return false;
}
