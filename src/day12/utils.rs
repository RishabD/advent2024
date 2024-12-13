use std::{
    fs::File,
    io::{self, BufRead},
};

pub const DIRECTIONS: &[(i32, i32)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn read_input() -> Vec<Vec<char>> {
    return io::BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
}

pub fn in_range(
    i: usize,
    j: usize,
    direction_index: usize,
    n: usize,
    m: usize,
) -> Option<(usize, usize)> {
    let (i_diff, j_diff) = DIRECTIONS[direction_index];
    if i as i32 + i_diff >= 0
        && i as i32 + i_diff < n as i32
        && j as i32 + j_diff >= 0
        && j as i32 + j_diff < m as i32
    {
        return Some(((i as i32 + i_diff) as usize, (j as i32 + j_diff) as usize));
    }
    return None;
}

pub fn in_range_vec(
    i: usize,
    j: usize,
    direction_indices: &Vec<usize>,
    n: usize,
    m: usize,
) -> Option<(usize, usize)> {
    let (mut i_diff, mut j_diff) = (0, 0);
    for &direction_index in direction_indices {
        let direction = DIRECTIONS[direction_index];
        i_diff += direction.0;
        j_diff += direction.1;
    }

    if i as i32 + i_diff >= 0
        && i as i32 + i_diff < n as i32
        && j as i32 + j_diff >= 0
        && j as i32 + j_diff < m as i32
    {
        return Some(((i as i32 + i_diff) as usize, (j as i32 + j_diff) as usize));
    }
    return None;
}
