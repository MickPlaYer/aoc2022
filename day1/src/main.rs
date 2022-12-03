use shared::read_file;

#[derive(Debug)]
struct Elf {
    calories: usize,
}

impl Elf {
    fn new(calories: usize) -> Self {
        Self { calories }
    }
}

fn main() {
    let content = read_file();
    let mut elves = Vec::new();
    collect_elves(content, &mut elves);
    let elf_for_sacks = looking_elf_for_snacks(&elves);
    println!("{:?}", elf_for_sacks);
    elves.sort_by(|a, b| b.calories.cmp(&a.calories));
    let mut lead_elves = Vec::new();
    for elf in elves.get(..3).unwrap() {
        lead_elves.push(elf);
    }
    println!(
        "{:?}\ntotal: {}",
        lead_elves,
        lead_elves.iter().map(|elf| elf.calories).sum::<usize>(),
    );
}

fn collect_elves(content: String, elves: &mut Vec<Elf>) {
    let mut calories = Vec::new();
    for line in content.lines() {
        match line.parse::<usize>() {
            Ok(calorie) => calories.push(calorie),
            Err(_) => {
                elves.push(Elf::new(calories.iter().sum()));
                calories.clear()
            }
        };
    }
    if !calories.is_empty() {
        elves.push(Elf::new(calories.iter().sum()));
    }
}

fn looking_elf_for_snacks(elves: &Vec<Elf>) -> Option<&Elf> {
    let mut elf_have_most_calories = elves.first();
    for elf in elves {
        match elf_have_most_calories {
            Some(current_elf) => {
                if elf.calories > current_elf.calories {
                    elf_have_most_calories = Some(elf)
                }
            }
            None => elf_have_most_calories = Some(elf),
        }
    }
    elf_have_most_calories
}
