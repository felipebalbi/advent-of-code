use std::collections::BTreeMap;

use anyhow::{Context, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    combinator::map,
    multi::fold_many1,
    sequence::{separated_pair, terminated, tuple},
    IResult,
};
use tracing::info;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

#[tracing::instrument(skip(input))]
fn directions(input: &str) -> IResult<&str, &str> {
    terminated(alpha1, line_ending)(input)
}

#[tracing::instrument(skip(input))]
fn node<'a>(input: &'a str) -> IResult<&'a str, (&'a str, Node<'a>)> {
    terminated(
        map(
            tuple((alpha1, tag(" = ("), alpha1, tag(", "), alpha1, tag(")"))),
            |(name, _, left, _, right, _)| (name, Node { left, right }),
        ),
        line_ending,
    )(input)
}

#[tracing::instrument(skip(input))]
fn nodes<'a>(input: &'a str) -> IResult<&str, BTreeMap<&str, Node<'a>>> {
    fold_many1(node, BTreeMap::new, |mut acc, (name, node)| {
        acc.insert(name, node);
        acc
    })(input)
}

#[tracing::instrument(skip(input))]
fn camel_map<'a>(input: &'a str) -> IResult<&'a str, (&str, BTreeMap<&str, Node<'a>>)> {
    separated_pair(directions, line_ending, nodes)(input)
}

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    info!("processing input");

    let (_, (directions, nodes)) = camel_map(input)?;
    let (mut name, mut node) = nodes
        .iter()
        .next()
        .expect("should have at least one element");

    let mut next_name = name;

    let result = directions
        .chars()
        .cycle()
        .try_fold(0, |mut acc, direction| {
            node = match direction {
                'L' => {
                    next_name = &&node.left;
                    nodes.get(node.left).expect("should have a valid left node")
                }
                'R' => {
                    next_name = &&node.right;
                    nodes
                        .get(node.right)
                        .expect("should have a valid right node")
                }
                _ => unreachable!(),
            };

            if name == &"ZZZ" {
                Err(acc)
            } else {
                name = next_name;
                acc += 1;
                Ok(acc)
            }
        })
        .unwrap_err();

    info!(?result);

    Ok(result.to_string())
}

#[tracing::instrument(skip(input))]
pub fn part1(input: &'static str) -> Result<String> {
    info!("part 1");

    process(input).context("process part 1")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_2_steps() {
        let input = r##"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"##;
        let result = process(input).unwrap();
        assert_eq!(result, "2");
    }

    #[test_log::test]
    fn test_6_steps() {
        let input = r##"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"##;
        let result = process(input).unwrap();
        assert_eq!(result, "6");
    }
}
