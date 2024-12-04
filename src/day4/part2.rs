use crate::day4::utils;

const CORNERS: &'static [(i32, i32)] = &[(1, 1), (1, -1), (-1, -1), (-1, 1)];

pub fn run() {
    let puzzle = utils::read_input();
    let n = puzzle.len() as i32;
    let m = puzzle[0].len() as i32;
    let total = (1..n - 1)
        .map(|i| {
            (1..m - 1)
                .map(|j| if word_search(&puzzle, i, j) { 1 } else { 0 })
                .sum::<i32>()
        })
        .sum::<i32>();
    println!("Total: {}", total);
}

fn word_search(puzzle: &Vec<Vec<char>>, i: i32, j: i32) -> bool {
    if puzzle[i as usize][j as usize] != 'A' {
        return false;
    }
    let mut m_count = 0;
    let mut s_count = 0;
    CORNERS.iter().for_each(|(i_diff, j_diff)| {
        if puzzle[(i + *i_diff) as usize][(j + *j_diff) as usize] == 'M' {
            m_count += 1;
        }
        if puzzle[(i + *i_diff) as usize][(j + *j_diff) as usize] == 'S' {
            s_count += 1;
        }
    });
    if m_count != 2 || s_count != 2 {
        return false;
    }
    return puzzle[(i + CORNERS[0].0) as usize][(j + CORNERS[0].1) as usize]
        != puzzle[(i + CORNERS[2].0) as usize][(j + CORNERS[2].1) as usize];
}
