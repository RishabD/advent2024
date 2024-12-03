use crate::day2::utils;

pub fn run() {
    let levels = utils::read_input();
    let count: i32 = levels
        .iter()
        .map(utils::level_is_safe)
        .map(|is_safe| if is_safe { 1 } else { 0 })
        .sum();
    println!("The number of safe levels is {}", count)
}
