use crate::structs::{Compartment, Rucksack};

pub fn find_share_items(context: &String) -> Vec<char> {
    let rucksacks = parse_rucksacks_part1(context);
    rucksacks
        .iter()
        .map(|rucksack| {
            rucksack
                .find_share_item()
                .expect(format!("No share item found in {:?}!", rucksack).as_str())
        })
        .collect()
}

fn parse_rucksacks_part1(context: &String) -> Vec<Rucksack> {
    let mut rucksacks = Vec::new();
    for line in context.lines() {
        let (first, last) = line.split_at(line.len() / 2);
        let compartments = vec![first, last]
            .iter()
            .map(|str| Compartment::new(str))
            .collect();
        rucksacks.push(Rucksack::new(compartments))
    }
    rucksacks
}
