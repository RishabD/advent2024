use regex::Regex;

use crate::day3::utils;

pub fn run() {
    let program = utils::read_input();
    let re = Regex::new(r"(mul\(\d+,\d+\))").unwrap();
    let total = program
        .iter()
        .map(|program_line| {
            re.captures_iter(&program_line)
                .map(|capture| utils::perform_mult(&capture[0]))
                .sum::<i32>()
        })
        .sum::<i32>();
    println!("The total of all mults is {}", total)
}
