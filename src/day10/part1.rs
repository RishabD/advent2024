use std::collections::HashSet;

use crate::day10::utils;

pub fn run() {
    let map = utils::read_input();
    let (n, m) = (map.len(), map[0].len());
    let mut visited: Vec<Vec<Option<HashSet<(usize, usize)>>>> =
        utils::create_visited::<HashSet<(usize, usize)>>(n, m);
    let mut total = 0;
    for i in 0..n {
        for j in 0..m {
            if map[i][j] == 0 {
                total += dfs(i, j, 9, &mut visited, &map).len();
            }
        }
    }
    println!("Total: {}", total);
}

fn dfs(
    i: usize,
    j: usize,
    end_value: u32,
    visited: &mut Vec<Vec<Option<HashSet<(usize, usize)>>>>,
    map: &Vec<Vec<u32>>,
) -> Vec<(usize, usize)> {
    if map[i][j] == end_value {
        if visited[i][j].is_none() {
            visited[i][j] = Some(HashSet::from([(i, j)]))
        }
        return vec![(i, j)];
    } else if let Some(reachable) = visited[i][j].as_ref() {
        return reachable.iter().map(|&position| position).collect();
    } else {
        let mut reachable = HashSet::new();
        for &(i_diff, j_diff) in utils::DIRECTIONS.iter() {
            if let Some((valid_i, valid_j)) =
                utils::in_range(i, j, i_diff, j_diff, map.len(), map[0].len())
            {
                if map[valid_i][valid_j] == map[i][j] + 1 {
                    for target_position in dfs(valid_i, valid_j, end_value, visited, map) {
                        if !reachable.contains(&target_position) {
                            reachable.insert(target_position);
                        }
                    }
                }
            }
        }
        visited[i][j] = Some(reachable);
        return visited[i][j]
            .as_ref()
            .unwrap()
            .iter()
            .map(|&position| position)
            .collect();
    }
}
