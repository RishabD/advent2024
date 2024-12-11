use std::{
    fs::File,
    io::{self, BufRead},
};

pub const DIRECTIONS: &[(i32, i32)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn read_input() -> Vec<Vec<u32>> {
    return io::BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u32)
                .collect()
        })
        .collect();
}

pub fn create_visited<T>(n: usize, m: usize) -> Vec<Vec<Option<T>>> {
    return (0..n).map(|_| (0..m).map(|_| None).collect()).collect();
}

pub fn in_range(
    i: usize,
    j: usize,
    i_diff: i32,
    j_diff: i32,
    n: usize,
    m: usize,
) -> Option<(usize, usize)> {
    if i as i32 + i_diff >= 0
        && i as i32 + i_diff < n as i32
        && j as i32 + j_diff >= 0
        && j as i32 + j_diff < m as i32
    {
        return Some(((i as i32 + i_diff) as usize, (j as i32 + j_diff) as usize));
    }
    return None;
}
