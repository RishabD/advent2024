use crate::day7::utils;

pub fn run() {
    let target_and_equations = utils::read_input();
    let total = target_and_equations
        .iter()
        .map(|(target, equation)| {
            if equation_can_reach_target(*target, equation[0], &equation[1..]) {
                *target
            } else {
                0
            }
        })
        .sum::<u64>();
    println!("Total: {}", total);
}

fn equation_can_reach_target(target: u64, running_total: u64, equation: &[u64]) -> bool {
    if equation.len() == 0 {
        return target == running_total;
    }
    return equation_can_reach_target(target, running_total + equation[0], &equation[1..])
        || equation_can_reach_target(target, running_total * equation[0], &equation[1..])
        || equation_can_reach_target(
            target,
            running_total * (10 as u64).pow(f64::log10(equation[0] as f64) as u32 + 1)
                + equation[0],
            &equation[1..],
        );
}
