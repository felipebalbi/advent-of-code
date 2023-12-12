use anyhow::{Context, Result};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, one_of, space0, space1},
    combinator::{map, recognize},
    multi::{many1, separated_list1},
    sequence::{pair, separated_pair, terminated, tuple},
    IResult,
};
use tracing::info;

#[derive(Debug, PartialEq, Clone)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug)]
struct Row {
    conditions: Vec<Condition>,
    groups: Vec<u32>,
}

#[tracing::instrument(skip(input))]
fn conditions(input: &str) -> IResult<&str, Vec<Condition>> {
    many1(map(one_of("?.#"), |c| match c {
        '?' => Condition::Unknown,
        '.' => Condition::Operational,
        '#' => Condition::Damaged,
        _ => unreachable!(),
    }))(input)
}

#[tracing::instrument(skip(input))]
fn groups(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(","), complete::u32)(input)
}

#[tracing::instrument(skip(input))]
fn row(input: &str) -> IResult<&str, Row> {
    map(
        separated_pair(conditions, space1, groups),
        |(conditions, groups)| Row { conditions, groups },
    )(input)
}

#[tracing::instrument(skip(input))]
fn records(input: &str) -> IResult<&str, Vec<Row>> {
    separated_list1(line_ending, row)(input)
}

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    info!("processing input");

    let (_, records) = records(input)?;

    info!(?records);

    let result = 0;

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
    fn one_arrangement() {
        let input = "???.### 1,1,3\n";
        let result = process(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "1");
    }

    #[test_log::test]
    fn four_arrangements() {
        let input = ".??..??...?##. 1,1,3\n";
        let result = process(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "4");
    }

    #[ignore]
    #[test_log::test]
    fn ten_arrangements() {
        let input = "?###???????? 3,2,1\n";
        let result = process(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "10");
    }

    #[ignore]
    #[test_log::test]
    fn twenty_one_arrangements() {
        let input = r##"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"##;
        let result = process(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "21");
    }
}
