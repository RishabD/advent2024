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
                            if other_i != i || other_j != j {
                                let (delta_i, delta_j) = diffs_post_simplification(
                                    (i as i32 - other_i as i32),
                                    (j as i32 - other_j as i32),
                                );
                                let mut pos_counter = 0;
                                while let Some((valid_i, valid_j)) = utils::in_range(
                                    (i as i32) + delta_i * pos_counter,
                                    (j as i32) + delta_j * pos_counter,
                                    n,
                                    m,
                                ) {
                                    antinode_mask[valid_i][valid_j] = true;
                                    pos_counter += 1;
                                }

                                let mut neg_counter = -1;
                                while let Some((valid_i, valid_j)) = utils::in_range(
                                    (i as i32) + delta_i * neg_counter,
                                    (j as i32) + delta_j * neg_counter,
                                    n,
                                    m,
                                ) {
                                    antinode_mask[valid_i][valid_j] = true;
                                    neg_counter -= 1;
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

fn diffs_post_simplification(change_in_i: i32, change_in_j: i32) -> (i32, i32) {
    if change_in_i == 0 {
        return (0, 1);
    }
    if change_in_j == 0 {
        return (1, 0);
    }
    let mut a = change_in_i.abs();
    let mut b = change_in_j.abs();
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    return (change_in_i / a, change_in_j / a);
}
