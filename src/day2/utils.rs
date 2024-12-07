use std::fs::File;
use std::io::{self, BufRead};

pub fn read_input() -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let reader = io::BufReader::new(File::open("input.txt").unwrap());
    for line in reader.lines() {
        let content = line.unwrap();
        let parts = content.split_whitespace();
        result.push(
            parts
                .map(|string_num| string_num.parse::<i32>().unwrap())
                .collect(),
        )
    }
    return result;
}

pub fn report_is_safe(report: &Vec<i32>) -> bool {
    return (strict_ascending(report) || strict_descending(report)) && valid_diffs(report);
}

fn strict_ascending(report: &Vec<i32>) -> bool {
    return report
        .iter()
        .zip(report.iter().skip(1))
        .all(|(first, second)| first < second);
}

fn strict_descending(report: &Vec<i32>) -> bool {
    return report
        .iter()
        .zip(report.iter().skip(1))
        .all(|(first, second)| first > second);
}

fn valid_diffs(report: &Vec<i32>) -> bool {
    return report
        .iter()
        .zip(report.iter().skip(1))
        .all(|(first, second)| first != second && (first - second).abs() <= 3);
}
