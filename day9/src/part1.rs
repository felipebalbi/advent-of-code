use anyhow::{Context, Result};
use itertools::Itertools;
use nom::{
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    IResult,
};
use std::ops::Not;
use tracing::info;

#[tracing::instrument(skip(input))]
fn history(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(space1, complete::i32)(input)
}

#[tracing::instrument(skip(input))]
fn oasis(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(line_ending, history)(input)
}

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    info!("processing input");

    let (_, oasis) = oasis(input)?;

    info!(?oasis);

    let result = oasis
        .into_iter()
        .map(|history| {
            std::iter::successors(Some(history), |measurements| {
                measurements.iter().all(|num| num == &0).not().then_some(
                    measurements
                        .iter()
                        .tuple_windows::<(&i32, &i32)>()
                        .map(|(left, right)| right - left)
                        .collect(),
                )
            })
            .map(|v| *v.last().unwrap())
            .sum::<i32>()
        })
        .sum::<i32>();

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
    fn it_works() {
        let input = r##"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"##;
        let result = process(input).unwrap();
        assert_eq!(result, "114");
    }
}
