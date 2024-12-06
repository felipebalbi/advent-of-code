use std::{cmp::Ordering, collections::BTreeMap};

use anyhow::{Context, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, line_ending},
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

fn gcd(a: usize, b: usize) -> usize {
    match (a, b) {
        (a, 0) => a,
        (0, b) => b,
        (a, b) => match a.cmp(&b) {
            Ordering::Less => gcd(a, b % a),
            _ => gcd(b, a % b),
        },
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[tracing::instrument(skip(input))]
fn directions(input: &str) -> IResult<&str, &str> {
    terminated(alphanumeric1, line_ending)(input)
}

#[tracing::instrument(skip(input))]
fn node<'a>(input: &'a str) -> IResult<&'a str, (&'a str, Node<'a>)> {
    terminated(
        map(
            tuple((
                alphanumeric1,
                tag(" = ("),
                alphanumeric1,
                tag(", "),
                alphanumeric1,
                tag(")"),
            )),
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
    let starting_points = nodes
        .iter()
        .filter(|(name, _)| name.ends_with('A'))
        .collect::<Vec<_>>();

    info!(?starting_points);

    let result = starting_points
        .into_iter()
        .map(|(starting_name, starting_node)| {
            let mut node = starting_node;
            let mut name = starting_name;
            let mut next_name = starting_name;

            directions
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

                    if name.ends_with('Z') {
                        Err(acc)
                    } else {
                        name = next_name;
                        acc += 1;
                        Ok(acc)
                    }
                })
                .unwrap_err()
        })
        .reduce(lcm)
        .unwrap();

    info!(?result);

    Ok(result.to_string())
}

#[tracing::instrument(skip(input))]
pub fn part2(input: &'static str) -> Result<String> {
    info!("part 2");

    process(input).context("process part 2")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_6_steps() {
        let input = r##"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"##;
        let result = process(input).unwrap();
        assert_eq!(result, "6");
    }
}
