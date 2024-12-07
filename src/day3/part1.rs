use regex::Regex;

use crate::day3::utils;

pub fn run() {
    let program = utils::read_input();
    let multiply_regex = Regex::new(r"(mul\(\d+,\d+\))").unwrap();
    let total = program
        .iter()
        .map(|program_line| {
            multiply_regex
                .captures_iter(&program_line)
                .map(|capture| utils::perform_multiply(&capture[0]))
                .sum::<u32>()
        })
        .sum::<u32>();
    println!("Total: {}", total)
}
