use regex::Regex;

use crate::day3::utils;

pub fn run() {
    let program = utils::read_input();
    let multiply_do_do_not_regex = Regex::new(r"(?:mul\((\d+),(\d+)\))|(?:don\'t)|(?:do)").unwrap();
    let mut multiply_enabled = true;
    let total = program
        .iter()
        .map(|program_line| {
            multiply_do_do_not_regex
                .captures_iter(&program_line)
                .map(|capture| match &capture[0] {
                    "do" => {
                        multiply_enabled = true;
                        return 0;
                    }
                    "don't" => {
                        multiply_enabled = false;
                        return 0;
                    }
                    _ => {
                        if multiply_enabled {
                            return utils::perform_multiply(&capture[0]);
                        } else {
                            return 0;
                        }
                    }
                })
                .sum::<u32>()
        })
        .sum::<u32>();
    println!("Total: {}", total)
}
