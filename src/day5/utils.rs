use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

pub fn read_input() -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let reader = io::BufReader::new(File::open("input.txt").unwrap());
    let mut reading_updates = false;
    let mut order_requirements: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();
    for line in reader.lines() {
        let content = line.unwrap();
        if content == "" {
            reading_updates = true;
        } else if reading_updates {
            updates.push(
                content
                    .split(',')
                    .map(|num| num.parse::<i32>().unwrap())
                    .collect(),
            );
        } else {
            let mut split_iterator = content.split('|');
            let first = split_iterator.next().unwrap().parse::<i32>().unwrap();
            let second = split_iterator.next().unwrap().parse::<i32>().unwrap();
            order_requirements.push((first, second));
        }
    }
    return (order_requirements, updates);
}

pub fn create_pages_to_pages_it_cannot_be_behind(
    order_requirements: &Vec<(i32, i32)>,
) -> HashMap<i32, HashSet<i32>> {
    let mut pages_to_pages_it_cannot_be_behind: HashMap<i32, HashSet<i32>> = HashMap::new();
    order_requirements
        .iter()
        .for_each(|(invalid_ahead_page, page)| {
            if pages_to_pages_it_cannot_be_behind.contains_key(&page) {
                pages_to_pages_it_cannot_be_behind
                    .get_mut(&page)
                    .unwrap()
                    .insert(*invalid_ahead_page);
            } else {
                pages_to_pages_it_cannot_be_behind.insert(*page, {
                    let mut look_ahead_set: HashSet<i32> = HashSet::new();
                    look_ahead_set.insert(*invalid_ahead_page);
                    look_ahead_set
                });
            }
        });
    return pages_to_pages_it_cannot_be_behind;
}

pub fn sort_update(
    update: &Vec<i32>,
    pages_to_pages_it_cannot_be_behind: &HashMap<i32, HashSet<i32>>,
) -> Vec<i32> {
    let mut modified_update = update.clone();
    modified_update.sort_by(|a, b| {
        if let Some(pages_it_cannot_be_behind_set) = pages_to_pages_it_cannot_be_behind.get(&a) {
            if pages_it_cannot_be_behind_set.contains(&b) {
                return Ordering::Equal;
            }
        }
        return Ordering::Less;
    });
    return modified_update;
}
