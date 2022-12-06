pub fn process_part1(content: String) -> Option<usize> {
    find_signal_with_length(content, 4)
}

pub fn process_part2(content: String) -> Option<usize> {
    find_signal_with_length(content, 14)
}

fn find_signal_with_length(content: String, length: usize) -> Option<usize> {
    for i in 0..(content.len() - length) {
        let chars = content.chars().skip(i).take(length).collect::<Vec<char>>();
        let mut is_all_uniq = true;
        for x in 0..length {
            for y in (x + 1)..length {
                is_all_uniq = is_all_uniq && chars[x] != chars[y];
            }
        }
        if is_all_uniq {
            return Some(i + length);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(6);
        let expects = vec![Some(7), Some(5), Some(6), Some(10), Some(11)];
        let answers = content
            .lines()
            .map(|line| process_part1(line.into()))
            .collect::<Vec<Option<usize>>>();
        assert_eq!(expects, answers);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(6);
        let answer = process_part1(content);
        assert_eq!(Some(1965), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample(6);
        let expects = vec![Some(19), Some(23), Some(23), Some(29), Some(26)];
        let answers = content
            .lines()
            .map(|line| process_part2(line.into()))
            .collect::<Vec<Option<usize>>>();
        assert_eq!(expects, answers);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(6);
        let answer = process_part2(content);
        assert_eq!(Some(2773), answer);
    }
}
