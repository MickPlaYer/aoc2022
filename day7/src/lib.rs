mod structs;

use structs::*;

pub fn process_part1(content: String) -> Option<usize> {
    let shell = parse_as_shell(content);
    Some(
        shell
            .search(&|size| -> bool { size <= 100000 })
            .iter()
            .map(|search_result| search_result.size)
            .sum::<usize>(),
    )
}

pub fn process_part2(content: String) -> Option<usize> {
    const TOTAL_SIZE: usize = 70000000;
    const SIZE_NEED: usize = 30000000;
    let shell = parse_as_shell(content);
    let current_space = TOTAL_SIZE - shell.root_size();
    let still_need = SIZE_NEED - current_space;
    let mut search_results = shell.search(&|size| -> bool { size >= still_need });
    search_results.sort();
    let size = search_results.get(0).unwrap().size;
    Some(size)
}

fn parse_as_shell(content: String) -> Shell {
    let flat_commands = FlatCommands::parse(content);
    let commands = flat_commands.compact();
    let mut shell = Shell::new();
    for command in commands.into_iter() {
        shell.apply_command(&command);
    }
    shell
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
        assert_eq!(Some(24933642), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(7);
        let answer = process_part2(content);
        assert_eq!(Some(5974547), answer);
    }
}
