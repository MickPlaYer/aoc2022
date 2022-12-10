#[derive(Debug)]
struct Tree {
    height: usize,
}

impl Tree {
    fn new(height: usize) -> Self {
        Self { height }
    }
}

struct Elements<T> {
    items: Vec<T>,
}

impl<T> Elements<T> {
    fn new(items: Vec<T>) -> Self {
        Self { items }
    }
}

impl<T> IntoIterator for Elements<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

struct ElementIterator<'a, T> {
    index: usize,
    size: usize,
    items: &'a Vec<T>,
}

impl<'a, T> ElementIterator<'a, T> {
    fn new(size: usize, items: &'a Vec<T>) -> Self {
        Self {
            index: 0,
            size,
            items,
        }
    }
}

impl<'a, T> Iterator for ElementIterator<'a, T> {
    type Item = Elements<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        let from = self.index * self.size;
        let to = from + self.size;
        let slice = self.items.get(from..to)?;
        self.index += 1;
        let mut items = Vec::new();
        for item in slice {
            items.push(item);
        }
        Some(Elements::new(items))
    }
}

#[derive(Debug)]
struct Grid<T> {
    we_size: usize,
    ns_size: usize,
    items: Vec<T>,
}

impl<T: Copy> Grid<T> {
    fn new_with_item(we_size: usize, ns_size: usize, item: T) -> Self {
        let mut items = Vec::new();
        for _ in 0..we_size * ns_size {
            items.push(item);
        }
        Self::new(we_size, ns_size, items)
    }
}

impl<T> Grid<T> {
    fn new(we_size: usize, ns_size: usize, items: Vec<T>) -> Self {
        Self {
            we_size,
            ns_size,
            items,
        }
    }

    fn rows(&self) -> ElementIterator<T> {
        ElementIterator::new(self.we_size, &self.items)
    }

    fn rotate(self) -> Self {
        let mut items = self
            .items
            .into_iter()
            .map(|item| Some(item))
            .collect::<Vec<Option<T>>>();
        let mut new_items = Vec::new();
        for x in 0..self.we_size {
            for y in 0..self.ns_size {
                let y = self.ns_size - y - 1;
                let index = x + y * self.we_size;
                let item = items.get_mut(index).unwrap().take().unwrap();
                new_items.push(item);
            }
        }
        Grid::new(self.we_size, self.ns_size, new_items)
    }

    fn iter(&self) -> std::slice::Iter<T> {
        self.items.iter()
    }
}

impl Grid<bool> {
    fn merge_with_or(&mut self, other: &Self) {
        let self_iter = self.items.iter();
        let other_iter = other.items.iter();
        self.items = self_iter
            .zip(other_iter)
            .map(|(a, b)| *a || *b)
            .collect::<Vec<bool>>();
    }
}

impl Grid<Tree> {
    fn visiable_trees_from_west(&self) -> Grid<bool> {
        let mut items = Vec::new();
        for row in self.rows() {
            let mut current_row_tallest_height: isize = -1;
            for tree in row {
                if current_row_tallest_height >= tree.height.try_into().unwrap() {
                    items.push(false);
                    continue;
                }
                current_row_tallest_height = tree.height.try_into().unwrap();
                items.push(true);
            }
        }
        Grid::new(self.we_size, self.ns_size, items)
    }
}

pub fn process_part1(content: String) -> Option<usize> {
    let mut trees = parse_trees(content);
    let mut visiable_trees = Grid::new_with_item(trees.we_size, trees.ns_size, false);
    for _ in 0..4 {
        let new_visiable_trees = &trees.visiable_trees_from_west();
        visiable_trees.merge_with_or(new_visiable_trees);
        trees = trees.rotate();
        visiable_trees = visiable_trees.rotate();
    }
    Some(visiable_trees.iter().filter(|e| **e).count())
}

fn parse_trees(content: String) -> Grid<Tree> {
    let we_size = content.lines().nth(0).expect("No any input data!").len();
    let ns_size = content.lines().count();
    let mut trees = Vec::new();
    for line in content.lines() {
        line.chars().for_each(|char| {
            let tree = Tree::new(
                char.to_string()
                    .parse::<usize>()
                    .expect(format!("Can not parse {} into number!", char).as_str()),
            );
            trees.push(tree);
        });
    }
    Grid::new(we_size, ns_size, trees)
}

pub fn process_part2(content: String) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 8;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn test_grid_rotte() {
        let grid = Grid::new(2, 2, vec![1, 2, 3, 4]).rotate();
        assert_eq!([3, 1, 4, 2], grid.items.as_slice());
    }

    #[test]
    fn test_grid_merge_with_or() {
        let mut grid_a = Grid::new(2, 2, vec![true, true, false, false]);
        let grid_b = Grid::new(2, 2, vec![true, false, true, false]);
        grid_a.merge_with_or(&grid_b);
        assert_eq!([true, true, true, false], grid_a.items.as_slice());
    }

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(21), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(1782), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(0), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(0), answer);
    }
}
