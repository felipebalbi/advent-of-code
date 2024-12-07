use anyhow::{Context, Result};
use nom::{
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use tracing::info;

#[tracing::instrument(skip(input))]
fn number_pair(input: &str) -> IResult<&str, (i64, i64)> {
    separated_pair(complete::i64, space1, complete::i64)(input)
}

#[tracing::instrument(skip(input))]
fn numbers(input: &str) -> IResult<&str, Vec<(i64, i64)>> {
    separated_list1(line_ending, number_pair)(input)
}

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    info!("processing input");

    let (_, numbers) = numbers(input)?;
    let (mut a, mut b): (Vec<i64>, Vec<i64>) = numbers.into_iter().unzip();

    a.sort();
    b.sort();

    let d = a
        .iter()
        .zip(b.iter())
        .map(|(a, b)| (b - a).abs())
        .sum::<i64>();

    Ok(d.to_string())
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
        let input = r##"3   4
4   3
2   5
1   3
3   9
3   3
"##;
        let result = process(input).unwrap();
        assert_eq!(result, "11");
    }
}
