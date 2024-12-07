use crate::day6::utils;

pub fn run() {
    let (mut map, (mut i, mut j), (n, m)) = utils::read_input();
    let mut direction_index = 0;
    let mut visited = utils::create_visited(n, m);
    while utils::coordinates_in_range(i, j, n, m) {
        visited[i as usize][j as usize][direction_index] = true;
        let direction = utils::DIRECTIONS[direction_index];
        if utils::coordinates_in_range(i + direction.0, j + direction.1, n, m)
            && map[(i + direction.0) as usize][(j + direction.1) as usize] != '#'
            && map[(i + direction.0) as usize][(j + direction.1) as usize] != 'O'
            && !visited[(i + direction.0) as usize][(j + direction.1) as usize]
                .iter()
                .any(|seen| *seen)
        {
            map[(i + direction.0) as usize][(j + direction.1) as usize] = '#';
            if detect_loop(i, j, n, m, direction_index, &map, &mut visited) {
                map[(i + direction.0) as usize][(j + direction.1) as usize] = 'O';
            } else {
                map[(i + direction.0) as usize][(j + direction.1) as usize] = '.';
            }
        }

        ((i, j), direction_index) = utils::get_next_position(i, j, n, m, direction_index, &map);
    }
    let total = map
        .iter()
        .map(|row| {
            row.iter()
                .map(|position| if *position == 'O' { 1 } else { 0 })
                .sum::<u32>()
        })
        .sum::<u32>();
    println!("Total {}", total);
}

fn detect_loop(
    current_i: i32,
    current_j: i32,
    n: i32,
    m: i32,
    current_direction_index: usize,
    map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<Vec<bool>>>,
) -> bool {
    let mut direction_index = current_direction_index;
    let mut simulation: Vec<((i32, i32), usize)> = Vec::new();
    let (mut i, mut j) = (current_i, current_j);
    loop {
        ((i, j), direction_index) = utils::get_next_position(i, j, n, m, direction_index, &map);
        if !utils::coordinates_in_range(i, j, n, m) {
            simulation
                .iter()
                .for_each(|((rem_i, rem_j), rem_direction_index)| {
                    visited[*rem_i as usize][*rem_j as usize][*rem_direction_index] = false;
                });
            return false;
        }
        if visited[i as usize][j as usize][direction_index] {
            simulation
                .iter()
                .for_each(|((rem_i, rem_j), rem_direction_index)| {
                    visited[*rem_i as usize][*rem_j as usize][*rem_direction_index] = false;
                });
            return true;
        }
        simulation.push(((i, j), direction_index));
        visited[i as usize][j as usize][direction_index] = true;
    }
}
