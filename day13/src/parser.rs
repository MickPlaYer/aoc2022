use crate::structs::Element;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space0},
    multi::separated_list0,
    sequence::{delimited, preceded},
    IResult,
};

fn atom(input: &str) -> IResult<&str, Element> {
    let (input, number) = preceded(space0, digit1)(input)?;
    Ok((input, Element::Atom(number.parse().unwrap())))
}

fn list(input: &str) -> IResult<&str, Element> {
    let (input, elements) = delimited(
        tag("["),
        separated_list0(delimited(space0, tag(","), space0), atom_or_list),
        tag("]"),
    )(input)?;
    Ok((input, Element::List(elements)))
}

fn atom_or_list(input: &str) -> IResult<&str, Element> {
    let (input, element) = alt((atom, list))(input)?;
    Ok((input, element))
}

pub fn parse(input: &str) -> Option<Element> {
    let (_, element) = atom_or_list(input).ok()?;
    Some(element)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atom_or_list() {
        let sample = "[1,[2,[3,[4,[5,6,7]]]],8,9]";
        let (_, element) = list(sample).unwrap();
        dbg!(&element);
        let expect = Element::List(vec![
            Element::Atom(1),
            Element::List(vec![
                Element::Atom(2),
                Element::List(vec![
                    Element::Atom(3),
                    Element::List(vec![
                        Element::Atom(4),
                        Element::List(vec![Element::Atom(5), Element::Atom(6), Element::Atom(7)]),
                    ]),
                ]),
            ]),
            Element::Atom(8),
            Element::Atom(9),
        ]);
        assert_eq!(expect, element)
    }
}
