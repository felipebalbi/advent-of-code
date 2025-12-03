use anyhow::{Context, Result};
use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending, one_of},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::separated_pair,
};
use tracing::info;

#[tracing::instrument(skip(input))]
fn battery(input: &str) -> IResult<&str, Vec<u64>> {
    many1(map(one_of("0123456789"), |c| {
        c.to_digit(10).unwrap() as u64
    }))
    .parse(input)
}

#[tracing::instrument(skip(input))]
fn batteries(input: &str) -> IResult<&str, Vec<Vec<u64>>> {
    separated_list1(line_ending, battery).parse(input)
}

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    info!("processing input");

    let (_, batteries) = batteries(input)?;

    let output: u64 = batteries
        .iter()
        .map(|bat| {
            bat.iter()
                .combinations(2)
                .map(|v| v[0] * 10 + v[1])
                .max()
                .unwrap_or(0)
        })
        .sum();

    Ok(output.to_string())
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
        let input = r##"987654321111111
811111111111119
234234234234278
818181911112111
"##;
        let result = process(input).unwrap();
        assert_eq!(result, "357");
    }
}
