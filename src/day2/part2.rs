use crate::day2::utils;

pub fn run() {
    let reports = utils::read_input();
    let total: i32 = reports
        .iter()
        .map(report_or_variants_is_safe)
        .map(|is_safe| if is_safe { 1 } else { 0 })
        .sum();
    println!("Total {}", total)
}

// Brute force scales for small number of levels
fn report_or_variants_is_safe(report: &Vec<i32>) -> bool {
    if utils::report_is_safe(report) {
        return true;
    }
    for i in 0..report.len() {
        let new_report: Vec<i32> = report
            .iter()
            .enumerate()
            .filter(|(index, _)| i != *index)
            .map(|(_, val)| *val)
            .collect();
        if utils::report_is_safe(&new_report) {
            return true;
        }
    }

    return false;
}
