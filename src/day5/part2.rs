use crate::day5::utils;

pub fn run() {
    let (order_requirements, updates) = utils::read_input();
    let pages_to_pages_it_cannot_be_behind =
        utils::create_pages_to_pages_it_cannot_be_behind(&order_requirements);
    let total = updates
        .iter()
        .map(|update| {
            let modified_update = utils::sort_update(&update, &pages_to_pages_it_cannot_be_behind);
            if modified_update
                .iter()
                .zip(update.iter())
                .any(|(a, b)| a != b)
            {
                return modified_update[modified_update.len() / 2];
            }
            return 0;
        })
        .sum::<i32>();
    println!("Total: {}", total);
}
