use crate::day9::utils;

pub fn run() {
    let disk_map = utils::read_input();
    let (mut mem_layout, _, empty_space) = utils::create_mem_layout(&disk_map);
    let mut file_width: usize = 0;
    let max_index = mem_layout.len() - 1;
    let mut used_index = max_index;
    while used_index > 0 && empty_space > 0 {
        if mem_layout[used_index].is_some()
            && (used_index == max_index
                || mem_layout[used_index + 1].is_none()
                || mem_layout[used_index].unwrap() != mem_layout[used_index + 1].unwrap())
        {
            file_width = 1;
        } else if mem_layout[used_index].is_some() {
            file_width += 1;
        }

        if mem_layout[used_index].is_some()
            && (used_index == 0
                || mem_layout[used_index - 1].is_none()
                || mem_layout[used_index].unwrap() != mem_layout[used_index - 1].unwrap())
        {
            if let Some(free_position) = find_first_contiguous_free_space_big_enough_for_width(
                &mem_layout,
                file_width,
                used_index,
            ) {
                for i in 0..file_width {
                    (mem_layout[free_position + i], mem_layout[used_index + i]) =
                        (mem_layout[used_index + i], mem_layout[free_position + i]);
                }
            }
        }
        used_index -= 1;
    }
    let total = utils::calculate_mem_layout_hash(&mem_layout);
    println!("Total: {}", total);
}

fn find_first_contiguous_free_space_big_enough_for_width(
    mem_layout: &Vec<Option<u64>>,
    file_width: usize,
    max_i: usize,
) -> Option<usize> {
    let mut free_spanning_region: (usize, usize) = (0, 0);
    for i in 0..mem_layout.len() {
        if i == max_i {
            return None;
        }
        if mem_layout[i].is_none() {
            free_spanning_region = (free_spanning_region.0, free_spanning_region.1 + 1);
        } else {
            free_spanning_region = (i + 1, 0);
        }

        if free_spanning_region.1 == file_width {
            return Some(free_spanning_region.0);
        }
    }
    return None;
}
