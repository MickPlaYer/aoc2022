use crate::structs::{Compartment, Group, Rucksack};

pub fn find_group_badges(context: &String) -> Vec<char> {
    let groups = parse_rucksacks_part2(context);
    groups
        .iter()
        .map(|group| {
            group
                .find_badge()
                .expect(format!("Can not find badge in {:?}!", group).as_str())
        })
        .collect()
}

fn parse_rucksacks_part2(context: &String) -> Vec<Group> {
    let mut groups = Vec::new();
    let binding = context.lines().collect::<Vec<&str>>();
    let chunks = binding.chunks(3);
    for lines in chunks {
        if lines.len() != 3 {
            panic!("Could not parse group don't have three lines!")
        }
        let rucksacks = lines
            .iter()
            .map(|line| Rucksack::new(vec![Compartment::new(line)]))
            .collect();
        groups.push(Group::new(rucksacks))
    }
    groups
}
