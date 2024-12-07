use std::{
    fs::File,
    io::{self, BufRead},
};

pub fn read_input() -> Vec<(u64, Vec<u64>)> {
    return io::BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| {
            let content = line.unwrap();
            let mut colon_split = content.split(": ");
            (colon_split.next().unwrap().parse::<u64>().unwrap(), {
                let space_split = colon_split.next().unwrap().split(" ");
                space_split
                    .into_iter()
                    .map(|equation_number_str| equation_number_str.parse::<u64>().unwrap())
                    .collect()
            })
        })
        .collect();
}
