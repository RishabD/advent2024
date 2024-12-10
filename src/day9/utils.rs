use std::{
    fs::File,
    io::{self, BufRead},
};

pub fn read_input() -> Vec<u64> {
    return io::BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();
}

pub fn create_mem_layout(disk_map: &Vec<u64>) -> (Vec<Option<u64>>, u64, u64) {
    let mut id: u64 = 0;
    let mut empty_space: u64 = 0;
    let mut used_space: u64 = 0;
    let mut mem_layout: Vec<Option<u64>> = Vec::new();
    for (i, &count) in disk_map.iter().enumerate() {
        if i % 2 == 0 {
            for _ in 0..count {
                mem_layout.push(Some(id));
            }
            used_space += count;
            id += 1;
        } else {
            for _ in 0..count {
                mem_layout.push(None);
            }
            empty_space += count;
        }
    }
    return (mem_layout, used_space, empty_space);
}

pub fn calculate_mem_layout_hash(mem_layout: &Vec<Option<u64>>) -> u64 {
    return mem_layout
        .iter()
        .enumerate()
        .map(
            |(i, mem)| {
                if let Some(id) = mem {
                    i as u64 * id
                } else {
                    0
                }
            },
        )
        .sum::<u64>();
}
