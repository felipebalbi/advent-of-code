use anyhow::{Context, Result};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::pair,
};
use tracing::info;

#[derive(Clone, Copy, Debug)]
enum Rotation {
    Left(i64),
    Right(i64),
}

#[tracing::instrument(skip(input))]
fn rotation(input: &str) -> IResult<&str, Rotation> {
    map(pair(alt((tag("L"), tag("R"))), complete::i64), |(c, n)| {
        if c == "L" {
            Rotation::Left(n)
        } else {
            Rotation::Right(n)
        }
    })
    .parse(input)
}

#[tracing::instrument(skip(input))]
fn rotations(input: &str) -> IResult<&str, Vec<Rotation>> {
    separated_list1(line_ending, rotation).parse(input)
}

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    info!("processing input");

    let (_, rotations) = rotations.parse(input)?;
    let mut dial = 50;
    let mut zero_count = 0;

    for rot in rotations {
        match rot {
            Rotation::Left(n) => {
                dial = (dial - n) % 100;
            }
            Rotation::Right(n) => {
                dial = (dial + n) % 100;
            }
        }

        if dial == 0 {
            zero_count += 1;
        }
    }

    Ok(zero_count.to_string())
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
    fn it_works() {
        let input = r##"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"##;
        let result = process(input).unwrap();
        assert_eq!(result, "3");
    }
}
