use regex::Regex;

use crate::day3::utils;

pub fn run() {
    let program = utils::read_input();
    let re = Regex::new(r"(?:mul\((\d+),(\d+)\))|(?:don\'t)|(?:do)").unwrap();
    let mut enabled = true;
    // let mut total = 0;
    let total = program
        .iter()
        .map(|program_line| {
            re.captures_iter(&program_line)
                .map(|capture| match &capture[0] {
                    "do" => {
                        enabled = true;
                        return 0;
                    }
                    "don't" => {
                        enabled = false;
                        return 0;
                    }
                    _ => {
                        if enabled {
                            return utils::perform_mult(&capture[0]);
                        } else {
                            return 0;
                        }
                    }
                })
                .sum::<i32>()
        })
        .sum::<i32>();
    println!("The total of all mults is {}", total)
}
