use crate::day2::utils;

pub fn run() {
    let levels = utils::read_input();
    let count: i32 = levels
        .iter()
        .map(safe)
        .map(|is_safe| if is_safe { 1 } else { 0 })
        .sum();
    println!("The number of safe levels is {}", count)
}

fn safe(level: &Vec<i32>) -> bool {
    if (strict_ascending(level) || strict_descending(level)) && valid_diffs(level) {
        return true;
    }
    for i in 0..level.len() {
        let new_level: Vec<i32> = level
            .iter()
            .enumerate()
            .filter(|(index, _)| i != *index)
            .map(|(_, val)| *val)
            .collect();
        if (strict_ascending(&new_level) || strict_descending(&new_level))
            && valid_diffs(&new_level)
        {
            return true;
        }
    }

    return false;
}

fn strict_ascending(level: &Vec<i32>) -> bool {
    return level
        .iter()
        .zip(level.iter().skip(1))
        .all(|(first, second)| first < second);
}

fn strict_descending(level: &Vec<i32>) -> bool {
    return level
        .iter()
        .zip(level.iter().skip(1))
        .all(|(first, second)| first > second);
}

fn valid_diffs(level: &Vec<i32>) -> bool {
    return level
        .iter()
        .zip(level.iter().skip(1))
        .all(|(first, second)| first != second && (first - second).abs() <= 3);
}
