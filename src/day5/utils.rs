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
    let mut page_updates: Vec<Vec<i32>> = Vec::new();
    for line in reader.lines() {
        let content = line.unwrap();
        if content == "" {
            reading_updates = true;
        } else if reading_updates {
            page_updates.push(
                content
                    .split(',')
                    .map(|num| num.parse::<i32>().unwrap())
                    .collect(),
            );
        } else {
            let mut split = content.split('|');
            let first = split.next().unwrap().parse::<i32>().unwrap();
            let second = split.next().unwrap().parse::<i32>().unwrap();
            order_requirements.push((first, second));
        }
    }
    return (order_requirements, page_updates);
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
                    let mut invalid_ahead_pages: HashSet<i32> = HashSet::new();
                    invalid_ahead_pages.insert(*invalid_ahead_page);
                    invalid_ahead_pages
                });
            }
        });
    return pages_to_pages_it_cannot_be_behind;
}

pub fn sort_update(
    update: &Vec<i32>,
    pages_to_pages_it_cannot_be_behind: &HashMap<i32, HashSet<i32>>,
) -> Vec<i32> {
    let mut new_update = update.clone();
    new_update.sort_by(|first_page, second_page| {
        if let Some(invalid_ahead_pages) = pages_to_pages_it_cannot_be_behind.get(&first_page) {
            if invalid_ahead_pages.contains(&second_page) {
                return Ordering::Equal;
            }
        }
        return Ordering::Less;
    });
    return new_update;
}
