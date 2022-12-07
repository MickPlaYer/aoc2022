mod structs;

use structs::*;

pub fn process_part1(content: String) -> Option<usize> {
    let flat_commands = FlatCommands::parse(content);
    let commands = flat_commands.compact();
    let mut shell = Shell::new();
    for command in commands.into_iter() {
        shell.apply_command(&command);
    }
    Some(
        shell
            .search(100000)
            .iter()
            .map(|search_result| search_result.size)
            .sum::<usize>(),
    )
}

pub fn process_part2(content: String) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(7);
        let answer = process_part1(content);
        assert_eq!(Some(95437), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(7);
        let answer = process_part1(content);
        assert_eq!(Some(1555642), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample(7);
        let answer = process_part2(content);
        assert_eq!(Some(0), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(7);
        let answer = process_part2(content);
        assert_eq!(Some(0), answer);
    }
}
