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
    let mult = if level[0] < level[level.len() - 1] {
        1
    } else {
        -1
    };
    for i in 1..level.len() {
        let first = level[i - 1];
        let second = level[i];
        if ((first - second).abs() == 0
            || (first - second).abs() > 3
            || first * mult >= second * mult)
        {
            return false;
        }
    }
    return true;
}
