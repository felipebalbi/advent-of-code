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
    let (a, b): (Vec<i64>, Vec<i64>) = numbers.into_iter().unzip();

    let d: i64 = a
        .iter()
        .map(|x| *x * b.iter().filter(|y| *y == x).count() as i64)
        .sum();

    Ok(d.to_string())
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
        let input = r##"3   4
4   3
2   5
1   3
3   9
3   3
"##;
        let result = process(input).unwrap();
        assert_eq!(result, "31");
    }
}
