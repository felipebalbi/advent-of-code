use std::ops::RangeInclusive;

use anyhow::{Context, Result};
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
};
use tracing::info;

#[tracing::instrument(skip(input))]
fn range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
    map(
        separated_pair(complete::u64, tag("-"), complete::u64),
        |(low, high)| RangeInclusive::new(low, high),
    )
    .parse(input)
}

#[tracing::instrument(skip(input))]
fn ranges(input: &str) -> IResult<&str, Vec<RangeInclusive<u64>>> {
    separated_list1(tag(","), range).parse(input)
}

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    info!("processing input");

    let (_, ranges) = ranges(input)?;

    let count = ranges
        .iter()
        .map(|range| {
            range
                .clone()
                .into_iter()
                .filter(|n| {
                    let s = n.to_string();
                    let len = s.len();

                    if !len.is_multiple_of(2) {
                        false
                    } else {
                        let (a, b) = s.split_at(len / 2);

                        a == b
                    }
                })
                .sum::<u64>()
        })
        .sum::<u64>();

    Ok(count.to_string())
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
        let input = r##"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
"##;
        let result = process(input).unwrap();
        assert_eq!(result, "1227775554");
    }
}
