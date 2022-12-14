use std::{cmp::Ordering, fmt::Debug};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Element {
    Atom(usize),
    List(Vec<Element>),
}

impl Element {
    pub fn to_list(&self) -> Element {
        match self {
            Element::Atom(_) => Element::List(vec![self.clone()]),
            Element::List(_) => self.clone(),
        }
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Atom(value) => std::fmt::Display::fmt(&value, f),
            Element::List(children) => {
                write!(f, "[")?;
                write!(
                    f,
                    "{}",
                    children
                        .iter()
                        .map(|child| format!("{}", child))
                        .collect::<Vec<String>>()
                        .join(",")
                )?;
                write!(f, "]")
            }
        }
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        let left = self;
        let right = other;
        match (&left, &right) {
            (Element::Atom(left), Element::Atom(right)) => left.cmp(&right),
            (Element::Atom(_), Element::List(_)) => Element::cmp(&left.to_list(), right),
            (Element::List(_), Element::Atom(_)) => Element::cmp(left, &right.to_list()),
            (Element::List(left), Element::List(right)) => compare_list_to_list(left, right),
        }
    }
}

fn compare_list_to_list(left: &Vec<Element>, right: &Vec<Element>) -> Ordering {
    let mut finger: usize = 0;
    loop {
        let left = left.get(finger);
        let right = right.get(finger);
        match (left, right) {
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(left), Some(right)) => {
                let ordering = Element::cmp(left, right);
                if !ordering.is_eq() {
                    return ordering;
                }
            }
        }
        finger += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parse;

    #[test]
    fn test_ord_empty_arrays() {
        let left = parse("[1]").unwrap();
        let right = parse("[[]]").unwrap();
        assert!(left.gt(&right))
    }

    #[test]
    fn test_ord_atoms() {
        let left = parse("[1,1,3,1,1]").unwrap();
        let right = parse("[1,1,5,1,1]").unwrap();
        assert!(left.lt(&right))
    }
}
