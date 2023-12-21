use anyhow::{Context, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, space0, space1},
    combinator::{map, recognize},
    multi::{many1, separated_list1},
    sequence::{pair, terminated, tuple},
    IResult,
};
use tracing::info;

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    info!("processing input");

    Ok("".to_string())
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
        let input = r##""##;
        let result = process(input).unwrap();
        assert_eq!(result, "42");
    }
}
