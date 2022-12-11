enum Command {
    Addx(isize),
    Noop,
}

impl Command {
    fn parse(input: &str) -> Command {
        if "noop" == input {
            Command::Noop
        } else if input.starts_with("addx") {
            let value = input
                .strip_prefix("addx ")
                .expect("Command addx need 1 argument!")
                .parse()
                .unwrap();
            Command::Addx(value)
        } else {
            panic!("Can not parse {} as command!", input)
        }
    }

    fn get_time(&self) -> usize {
        match self {
            Command::Addx(_) => 2,
            Command::Noop => 1,
        }
    }
}

struct Cpu {
    commands: Vec<Command>,
    cycle_number: isize,
    register: isize,
}

impl Cpu {
    fn new(commands: Vec<Command>) -> Self {
        Cpu {
            commands,
            cycle_number: 0,
            register: 1,
        }
    }

    fn run<F>(&mut self, mut monitor_handler: F)
    where
        F: FnMut(isize, isize),
    {
        let commands = &mut self.commands;
        commands.reverse();
        let mut last_command = commands.pop().unwrap();
        let mut command_time = last_command.get_time();
        loop {
            self.cycle_number += 1;
            command_time -= 1;
            monitor_handler(self.cycle_number, self.register);
            if command_time == 0 {
                if let Command::Addx(value) = last_command {
                    self.register += value
                }
            } else {
                continue;
            }
            if commands.is_empty() {
                break;
            }
            last_command = commands.pop().unwrap();
            command_time = last_command.get_time();
        }
    }

    fn run_and_monitor_interesting_signal_strengths(&mut self) -> isize {
        let mut result = 0;
        self.run(|cycle_number, register| {
            if cycle_number % 40 == 20 {
                result += cycle_number * register
            }
        });
        result
    }
}

pub fn process_part1(content: String) -> Option<isize> {
    let commands = content.lines().map(|line| Command::parse(line)).collect();
    let mut cpu = Cpu::new(commands);
    let result = cpu.run_and_monitor_interesting_signal_strengths();
    Some(result)
}

pub fn process_part2(content: String) -> String {
    let commands = content.lines().map(|line| Command::parse(line)).collect();
    let mut cpu = Cpu::new(commands);
    let mut screen = Vec::new();
    cpu.run(|cycle_number, register| {
        if screen.len() >= 240 {
            return;
        }
        let crt_column = (cycle_number - 1) % 40;
        let sprite_range = register - 1..=register + 1;
        if sprite_range.contains(&crt_column) {
            screen.push("#")
        } else {
            screen.push(".")
        }
    });
    screen.join("")
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 10;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(13140), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(14560), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content);
        let expected = "\
            ##..##..##..##..##..##..##..##..##..##..\
            ###...###...###...###...###...###...###.\
            ####....####....####....####....####....\
            #####.....#####.....#####.....#####.....\
            ######......######......######......####\
            #######.......#######.......#######.....\
        ";
        assert_eq!(expected, answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        let expected = "\
            ####.#..#.###..#..#.####.###..#..#.####.\
            #....#.#..#..#.#..#.#....#..#.#..#....#.\
            ###..##...#..#.####.###..#..#.#..#...#..\
            #....#.#..###..#..#.#....###..#..#..#...\
            #....#.#..#.#..#..#.#....#....#..#.#....\
            ####.#..#.#..#.#..#.####.#.....##..####.\
        ";
        assert_eq!(expected, answer);
    }
}
