use crate::day12::utils;

pub fn run() {
    let map = utils::read_input();
    let (n, m) = (map.len(), map[0].len());
    let mut visited: Vec<Vec<Option<u32>>> = vec![vec![None; m]; n];
    let mut total = 0;
    for i in 0..n {
        for j in 0..m {
            if visited[i][j].is_none() {
                let mut plants_inside_fence: Vec<(usize, usize)> = Vec::new();
                search(
                    i,
                    j,
                    n,
                    m,
                    map[i][j],
                    &map,
                    &mut visited,
                    &mut plants_inside_fence,
                );
                total += (plants_inside_fence.len() as u32)
                    * plants_inside_fence
                        .iter()
                        .map(|&(visited_i, visited_j)| {
                            utils::DIRECTIONS.len() as u32 - visited[visited_i][visited_j].unwrap()
                        })
                        .sum::<u32>()
            }
        }
    }

    println!("Total: {}", total);
}

fn search(
    i: usize,
    j: usize,
    n: usize,
    m: usize,
    plant: char,
    map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<Option<u32>>>,
    group: &mut Vec<(usize, usize)>,
) {
    let mut adj_count = 0;
    visited[i][j] = Some(0);
    group.push((i, j));
    for direction_index in 0..utils::DIRECTIONS.len() {
        if let Some((valid_i, valid_j)) = utils::in_range(i, j, direction_index, n, m) {
            if map[valid_i][valid_j] == plant {
                adj_count += 1;
                if visited[valid_i][valid_j].is_none() {
                    search(valid_i, valid_j, n, m, plant, map, visited, group);
                }
            }
        }
    }
    visited[i][j] = Some(adj_count);
}
