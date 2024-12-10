use crate::day9::utils;

pub fn run() {
    let disk_map = utils::read_input();
    let (mut mem_layout, _, empty_space) = utils::create_mem_layout(&disk_map);
    let max_index = mem_layout.len() - 1;
    let mut index = max_index;
    let mut free_index: usize = 0;
    while index > 0 && empty_space > 0 {
        if mem_layout[index].is_some() {
            while mem_layout[free_index].is_some() && free_index <= max_index {
                free_index += 1;
            }
            if free_index > index {
                break;
            }
            (mem_layout[index], mem_layout[free_index]) =
                (mem_layout[free_index], mem_layout[index])
        }
        index -= 1;
    }
    let total = utils::calculate_mem_layout_hash(&mem_layout);
    println!("Total: {}", total);
}
