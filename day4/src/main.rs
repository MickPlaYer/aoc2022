use shared::read_file;

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn is_fully_contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

#[derive(Debug)]
struct RangePair {
    left: Range,
    right: Range,
}

impl RangePair {
    fn is_any_fully_contains(&self) -> bool {
        self.left.is_fully_contains(&self.right) || self.right.is_fully_contains(&self.left)
    }

    fn is_overlap(&self) -> bool {
        self.left.start <= self.right.end && self.left.end >= self.right.start
    }
}

fn main() {
    let context = read_file();
    let range_pairs = context
        .lines()
        .map(|line| parse_line(line).expect(format!("Fail to parse line: {}!", line).as_str()))
        .collect::<Vec<RangePair>>();
    // Part 1
    let any_fully_contains_range_pairs = range_pairs
        .iter()
        .filter(|range_pair| range_pair.is_any_fully_contains())
        .collect::<Vec<&RangePair>>();
    println!(
        "Found any_fully_contains_range_pairs: {:?}\ntotal {} pairs",
        any_fully_contains_range_pairs,
        any_fully_contains_range_pairs.len()
    );
    // Part 2
    let overlap_range_pairs = range_pairs
        .iter()
        .filter(|range_pair| range_pair.is_overlap())
        .collect::<Vec<&RangePair>>();
    println!(
        "Found overlap_range_pairs: {:?}\ntotal {} pairs",
        overlap_range_pairs,
        overlap_range_pairs.len()
    );
}

fn parse_line(line: &str) -> Option<RangePair> {
    let mut binding = line.split(',');
    let left = binding.next()?;
    let right = binding.next()?;
    let left = convert_to_range(left)?;
    let right = convert_to_range(right)?;
    Some(RangePair { left, right })
}

fn convert_to_range(text: &str) -> Option<Range> {
    let mut binding = text.split('-');
    let start = binding.next()?.parse::<usize>().ok()?;
    let end = binding.next()?.parse::<usize>().ok()?;
    Some(Range { start, end })
}
