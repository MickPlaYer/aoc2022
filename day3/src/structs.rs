#[derive(Debug)]
pub struct Compartment {
    items: Vec<char>,
}

impl Compartment {
    pub fn new(str: &str) -> Self {
        let mut items = str.chars().collect::<Vec<char>>();
        items.sort();
        Self { items }
    }
}

#[derive(Debug)]
pub struct Rucksack {
    compartments: Vec<Compartment>,
}

impl Rucksack {
    pub fn new(compartments: Vec<Compartment>) -> Self {
        Self { compartments }
    }

    pub fn find_share_item(&self) -> Option<char> {
        for x in &self.compartments[0].items {
            for y in &self.compartments[1].items {
                if x == y {
                    return Some(*x);
                }
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct Group {
    rucksacks: Vec<Rucksack>,
}

impl Group {
    pub fn new(rucksacks: Vec<Rucksack>) -> Self {
        Self { rucksacks }
    }

    pub fn find_badge(&self) -> Option<char> {
        // It's fine 🔥🐶☕🔥
        for x in &self.rucksacks[0].compartments[0].items {
            for y in &self.rucksacks[1].compartments[0].items {
                for z in &self.rucksacks[2].compartments[0].items {
                    if x == y && y == z {
                        return Some(*x);
                    }
                }
            }
        }
        None
    }
}
