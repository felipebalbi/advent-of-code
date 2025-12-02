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
    let mut crossings = 0;

    for rot in rotations {
        match rot {
            Rotation::Left(n) => {
                crossings += n / 100;
                let remainder = n % 100;

                if dial > 0 && (dial - remainder) <= 0 {
                    crossings += 1;
                }

                dial -= remainder;
                dial = dial.rem_euclid(100);
            }
            Rotation::Right(n) => {
                crossings += n / 100;
                let remainder = n % 100;

                dial += remainder;

                if dial >= 100 {
                    crossings += 1;
                }
                dial = dial.rem_euclid(100);
            }
        }
    }

    Ok(crossings.to_string())
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
        assert_eq!(result, "6");
    }

    #[test_log::test]
    fn r1000() {
        let input = r##"L50
L50
L100
L150"##;
        let result = process(input).unwrap();
        assert_eq!(result, "4");
    }
}
