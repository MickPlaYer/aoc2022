use regex::Regex;
use shared::read_file;

#[derive(Debug)]
struct Crate {
    name: char,
}

#[derive(Debug)]
struct Stack {
    crates: Vec<Crate>,
}

impl Stack {
    fn new() -> Self {
        Self { crates: Vec::new() }
    }

    fn top_crate_name(&self) -> String {
        let binding = self.crates.last();
        if binding.is_none() {
            return String::from(" ");
        }
        binding.unwrap().name.to_string()
    }
}

#[derive(Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn parse(line: &str) -> Option<Self> {
        let pattern =
            Regex::new(r"move (?P<amount>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
        let captures = pattern.captures(line)?;
        let amount = captures["amount"].parse().ok()?;
        let from = captures["from"].parse().ok()?;
        let to = captures["to"].parse().ok()?;
        Some(Self { amount, from, to })
    }
}

enum CrateMover {
    CM9000,
    CM9001,
}

fn main() {
    let context = read_file();
    let stacks = work_with_crate_mover(&context, CrateMover::CM9000);
    println!("Part 1 answer is: {:?}", get_top_crate_names(stacks));
    let stacks = work_with_crate_mover(&context, CrateMover::CM9001);
    println!("Part 2 answer is: {:?}", get_top_crate_names(stacks));
}

fn work_with_crate_mover(context: &String, crate_mover: CrateMover) -> Vec<Stack> {
    let lines = context.lines().collect::<Vec<&str>>();
    let split_index = lines
        .iter()
        .position(|line| line == &"")
        .expect("Could not find empty line to split cargos and moves!");
    let (cargo_lines, move_lines) = lines.split_at(split_index);
    let mut stacks = parse_stacks(cargo_lines);
    let moves = parse_moves(move_lines);
    for a_move in moves {
        apply_move(&crate_mover, &mut stacks, &a_move);
    }
    stacks
}

fn parse_stacks(cargo_lines: &[&str]) -> Vec<Stack> {
    let cargo_lines = cargo_lines.iter().rev().map(|e| *e).collect::<Vec<&str>>();
    let (stacks_line, cargo_lines) = cargo_lines
        .split_first()
        .expect("Not enough lines in cargo_lines");
    let binding = stacks_line.chars().collect::<Vec<char>>();
    let chunks = binding.chunks(4);
    let mut stacks = Vec::new();
    for chunk in chunks {
        let id = chunk[1];
        if id != ' ' {
            stacks.push(Stack::new())
        }
    }
    for line in cargo_lines.iter() {
        let binding = line.chars().collect::<Vec<char>>();
        let chunks = binding.chunks(4);
        for (index, chunk) in chunks.enumerate() {
            let name = chunk[1];
            if name != ' ' {
                stacks[index].crates.push(Crate { name });
            }
        }
    }
    stacks
}

fn parse_moves(move_lines: &[&str]) -> Vec<Move> {
    let mut moves = Vec::new();
    for line in move_lines {
        if let Some(m) = Move::parse(line) {
            moves.push(m);
        }
    }
    moves
}

fn apply_move(crate_mover: &CrateMover, stacks: &mut Vec<Stack>, a_move: &Move) -> Option<()> {
    let mut temp = Vec::new();
    {
        let from = stacks.get_mut(a_move.from - 1)?;
        for _ in 0..a_move.amount {
            temp.push(
                from.crates
                    .pop()
                    .expect("Could not move without any crate!"),
            )
        }
    }
    if let CrateMover::CM9001 = crate_mover {
        temp.reverse()
    }
    let to = stacks.get_mut(a_move.to - 1)?;
    to.crates.append(&mut temp);
    Some(())
}

fn get_top_crate_names(stacks: Vec<Stack>) -> String {
    let top_crate_names = stacks
        .iter()
        .map(|stack| stack.top_crate_name())
        .collect::<Vec<String>>()
        .join("");
    top_crate_names
}
