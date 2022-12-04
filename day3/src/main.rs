use day3::*;
use shared::read_file;

fn main() {
    let context = read_file();
    // Part 1
    let share_items = find_share_items(&context);
    println!(
        "Found share_items {:?} and the sum is {}",
        share_items,
        get_sum(&share_items)
    );
    // Part 2
    let badges = find_group_badges(&context);
    println!(
        "Found badges {:?} and the sum is {}",
        badges,
        get_sum(&badges)
    );
}

fn get_sum(items: &Vec<char>) -> usize {
    const LOWERCASE_START: usize = 1;
    const UPPERCASE_START: usize = 27;
    items
        .iter()
        .map(|c| match c {
            'a'..='z' => *c as usize - 'a' as usize + LOWERCASE_START,
            'A'..='Z' => *c as usize - 'A' as usize + UPPERCASE_START,
            _ => 0,
        })
        .sum()
}
