use crate::day4::utils;

const DIRECTIONS: &'static [(i32, i32)] = &[
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
    (1, 1),
    (-1, 1),
    (1, -1),
    (-1, -1),
];

pub fn run() {
    let puzzle = utils::read_input();
    let n = puzzle.len() as i32;
    let m = puzzle[0].len() as i32;
    let total = (0..n)
        .map(|i| {
            (0..m)
                .map(|j| {
                    DIRECTIONS
                        .iter()
                        .map(|direction| {
                            if word_search(&puzzle, "XMAS", i, j, n, m, *direction) {
                                1
                            } else {
                                0
                            }
                        })
                        .sum::<i32>()
                })
                .sum::<i32>()
        })
        .sum::<i32>();
    println!("Total: {}", total);
}

fn word_search(
    puzzle: &Vec<Vec<char>>,
    word: &str,
    i: i32,
    j: i32,
    n: i32,
    m: i32,
    direction: (i32, i32),
) -> bool {
    if word.len() == 0 {
        return true;
    }
    if i < 0
        || i >= n
        || j < 0
        || j >= m
        || puzzle[i as usize][j as usize] != word.chars().next().unwrap()
    {
        return false;
    }
    return word_search(
        &puzzle,
        &word[1..],
        i + direction.0,
        j + direction.1,
        n,
        m,
        direction,
    );
}
