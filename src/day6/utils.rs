use std::{
    fs::File,
    io::{self, BufRead},
};
pub const DIRECTIONS: &[(i32, i32)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn read_input() -> (Vec<Vec<char>>, (i32, i32), (i32, i32)) {
    let map: Vec<Vec<char>> = io::BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect();
    let dims = (map.len() as i32, map[0].len() as i32);
    let guard_location = find_guard(dims.0, dims.1, &map);
    return (map, guard_location, dims);
}

pub fn create_visited(n: i32, m: i32) -> Vec<Vec<Vec<bool>>> {
    return (0..n)
        .map(|_| (0..m).map(|_| vec![false, false, false, false]).collect())
        .collect();
}

fn find_guard(n: i32, m: i32, map: &Vec<Vec<char>>) -> (i32, i32) {
    {
        for i in 0..n {
            for j in 0..m {
                if map[i as usize][j as usize] == '^' {
                    return (i, j);
                }
            }
        }
        panic!("No guard found");
    }
}

pub fn coordinates_in_range(i: i32, j: i32, n: i32, m: i32) -> bool {
    return i >= 0 && i < n && j >= 0 && j < m;
}

pub fn get_next_position(
    i: i32,
    j: i32,
    n: i32,
    m: i32,
    direction_index: usize,
    map: &Vec<Vec<char>>,
) -> ((i32, i32), usize) {
    let direction = DIRECTIONS[direction_index];
    if coordinates_in_range(i + direction.0, j + direction.1, n, m)
        && map[(i + direction.0) as usize][(j + direction.1) as usize] == '#'
    {
        return ((i, j), (direction_index + 1) % DIRECTIONS.len());
    }
    return ((i + direction.0, j + direction.1), direction_index);
}
