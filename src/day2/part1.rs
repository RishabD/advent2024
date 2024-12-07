use crate::day2::utils;

pub fn run() {
    let reports = utils::read_input();
    let total: i32 = reports
        .iter()
        .map(utils::report_is_safe)
        .map(|is_safe| if is_safe { 1 } else { 0 })
        .sum();
    println!("Total: {}", total)
}
