use crate::day10::utils;

pub fn run() {
    let map = utils::read_input();
    let (n, m) = (map.len(), map[0].len());
    let mut visited = utils::create_visited(n, m);
    let mut total = 0;
    for i in 0..n {
        for j in 0..m {
            if map[i][j] == 0 {
                let add = dfs(i, j, 9, &mut visited, &map);
                total += add;
            }
        }
    }
    println!("Total: {}", total);
}

fn dfs(
    i: usize,
    j: usize,
    end_value: u32,
    visited: &mut Vec<Vec<Option<u32>>>,
    map: &Vec<Vec<u32>>,
) -> u32 {
    if map[i][j] == end_value {
        visited[i][j] = Some(1);
        return 1;
    } else if let Some(already_visited) = visited[i][j] {
        return already_visited;
    } else {
        let mut total_ways = 0;
        for &(i_diff, j_diff) in utils::DIRECTIONS.iter() {
            if let Some((valid_i, valid_j)) =
                utils::in_range(i, j, i_diff, j_diff, map.len(), map[0].len())
            {
                if map[valid_i][valid_j] == map[i][j] + 1 {
                    total_ways += dfs(valid_i, valid_j, end_value, visited, map);
                }
            }
        }
        visited[i][j] = Some(total_ways);
        return total_ways;
    }
}
