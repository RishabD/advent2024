use crate::day8::utils;

pub fn run() {
    let map = utils::read_input();
    let (n, m) = (map.len(), map[0].len());
    let frequency_to_positions = utils::create_frequency_to_positions(&map);
    let mut antinode_mask = utils::create_antinode_mask(n, m);

    map.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, frequency)| {
            if *frequency != '.' {
                if let Some(matching_frequency_positions) = frequency_to_positions.get(frequency) {
                    matching_frequency_positions
                        .iter()
                        .for_each(|&(other_i, other_j)| {
                            if i != other_i || j != other_j {
                                if let Some((valid_i, valid_j)) = utils::in_range(
                                    2 * i as i32 - other_i as i32,
                                    2 * j as i32 - other_j as i32,
                                    n,
                                    m,
                                ) {
                                    antinode_mask[valid_i][valid_j] = true;
                                }
                            }
                        })
                }
            }
        })
    });

    let total = antinode_mask
        .iter()
        .map(|row| {
            row.iter()
                .map(|&value| if value { 1 } else { 0 })
                .sum::<u32>()
        })
        .sum::<u32>();
    println!("Total: {}", total);
}
