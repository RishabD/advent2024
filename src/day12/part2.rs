use crate::day12::utils;

pub fn run() {
    let map = utils::read_input();
    let (n, m) = (map.len(), map[0].len());
    let mut visited: Vec<Vec<bool>> = vec![vec![false; m]; n];
    let mut total = 0;
    for i in 0..n {
        for j in 0..m {
            if !visited[i][j] {
                let (area, num_sides) = search(i, j, n, m, map[i][j], &map, &mut visited);
                total += area * num_sides;
            }
        }
    }
    println!("Total: {}", total);
}

const CORNER: &[(usize, usize)] = &[(0, 1), (1, 2), (2, 3), (3, 0)];

fn is_same(valid_coords: Option<(usize, usize)>, plant: char, map: &Vec<Vec<char>>) -> bool {
    if let Some((valid_i, valid_j)) = valid_coords {
        return map[valid_i][valid_j] == plant;
    }
    return false;
}

fn search(
    i: usize,
    j: usize,
    n: usize,
    m: usize,
    plant: char,
    map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
) -> (u32, u32) {
    let mut num_sides = 0;
    let mut area: u32 = 1;
    visited[i][j] = true;
    for &(direction_index_first, direction_index_second) in CORNER.iter() {
        if !is_same(
            utils::in_range(i, j, direction_index_first, n, m),
            plant,
            map,
        ) && !is_same(
            utils::in_range(i, j, direction_index_second, n, m),
            plant,
            map,
        ) {
            num_sides += 1;
        }

        if is_same(
            utils::in_range(i, j, direction_index_first, n, m),
            plant,
            map,
        ) && is_same(
            utils::in_range(i, j, direction_index_second, n, m),
            plant,
            map,
        ) && !is_same(
            utils::in_range_vec(
                i,
                j,
                &vec![direction_index_first, direction_index_second],
                n,
                m,
            ),
            plant,
            map,
        ) {
            num_sides += 1;
        }
    }

    for direction_index in 0..utils::DIRECTIONS.len() {
        if let Some((valid_i, valid_j)) = utils::in_range(i, j, direction_index, n, m) {
            if map[valid_i][valid_j] == plant && !visited[valid_i][valid_j] {
                let (other_area, other_num_sides) =
                    search(valid_i, valid_j, n, m, plant, map, visited);
                area += other_area;
                num_sides += other_num_sides;
            }
        }
    }

    return (area, num_sides);
}
