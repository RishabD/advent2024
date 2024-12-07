use crate::day6::utils;

pub fn run() {
    let (mut map, (mut i, mut j), (n, m)) = utils::read_input();
    let mut direction_index = 0;
    while utils::coordinates_in_range(i, j, n, m) {
        map[i as usize][j as usize] = 'X';
        ((i, j), direction_index) = utils::get_next_position(i, j, n, m, direction_index, &map);
    }
    let total = map
        .iter()
        .map(|row| {
            row.iter()
                .map(|position| if *position == 'X' { 1 } else { 0 })
                .sum::<u32>()
        })
        .sum::<u32>();
    println!("Total: {}", total);
}
