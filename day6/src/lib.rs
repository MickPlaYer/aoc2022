pub fn process_part1(content: String) -> Option<usize> {
    const SIGN_LENGTH: usize = 4;
    for i in 0..(content.len() - SIGN_LENGTH) {
        let chars = content
            .chars()
            .skip(i)
            .take(SIGN_LENGTH)
            .collect::<Vec<char>>();
        let mut is_all_uniq = true;
        for x in 0..SIGN_LENGTH {
            for y in (x + 1)..SIGN_LENGTH {
                is_all_uniq = is_all_uniq && chars[x] != chars[y];
            }
        }
        if is_all_uniq {
            return Some(i + SIGN_LENGTH);
        }
    }
    None
}

pub fn process_part2(content: String) -> Option<usize> {
    Some(0)
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
    #[ignore = "for now"]
    fn process_part2_with_sample() {
        let content = read_sample(6);
        let result = process_part2(content);
        assert_eq!(result, None);
    }
}
