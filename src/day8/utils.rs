use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

pub fn read_input() -> Vec<Vec<char>> {
    return io::BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|row| row.unwrap().chars().collect())
        .collect();
}

pub fn create_frequency_to_positions(map: &Vec<Vec<char>>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut frequency_to_position = HashMap::new();
    map.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, &frequency)| {
            if frequency != '.' {
                (*frequency_to_position.entry(frequency).or_insert(Vec::new())).push((i, j));
            }
        })
    });
    return frequency_to_position;
}

pub fn create_antinode_mask(n: usize, m: usize) -> Vec<Vec<bool>> {
    return (0..n).map(|_| (0..m).map(|_| false).collect()).collect();
}

pub fn in_range(i: i32, j: i32, n: usize, m: usize) -> Option<(usize, usize)> {
    if i >= 0 && i < n as i32 && j >= 0 && j < m as i32 {
        return Some((i as usize, j as usize));
    }
    return None;
}
