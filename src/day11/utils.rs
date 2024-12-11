use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

pub fn read_input() -> Vec<u64> {
    return io::BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|num| num.to_string().parse::<u64>().unwrap())
        .collect();
}

pub fn get_final_stone_count_from_stone(
    stone: u64,
    iter: usize,
    max_iter: usize,
    cache: &mut HashMap<u64, Vec<Option<u64>>>,
) -> u64 {
    if iter == max_iter {
        return 1;
    }

    let value_to_record: u64 = {
        let cache_entry = cache.entry(stone).or_insert(vec![None; max_iter]);
        if let Some(recorded_value) = cache_entry[iter] {
            return recorded_value;
        }
        if stone == 0 {
            get_final_stone_count_from_stone(1, iter + 1, max_iter, cache)
        } else if (f64::log10(stone as f64) as u64 + 1) % 2 == 0 {
            let divisor = (10 as u64).pow((f64::log10(stone as f64) as u32 + 1) / 2);
            let left_half = stone / divisor;
            let right_half = stone % divisor;
            get_final_stone_count_from_stone(left_half, iter + 1, max_iter, cache)
                + get_final_stone_count_from_stone(right_half, iter + 1, max_iter, cache)
        } else {
            get_final_stone_count_from_stone(stone * 2024, iter + 1, max_iter, cache)
        }
    };

    let cache_entry = cache.entry(stone).or_insert(vec![None; max_iter]);
    cache_entry[iter] = Some(value_to_record);
    return value_to_record;
}
