use anyhow::{Context, Result};
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{line_ending, space1},
    combinator::map,
    sequence::{pair, preceded, separated_pair},
    IResult,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tracing::info;

#[derive(Debug)]
struct Sheet {
    time: u64,
    distance: u64,
}

#[tracing::instrument(skip(input))]
fn number(input: &str) -> IResult<&str, u64> {
    map(take_until("\n"), |s: &str| {
        let mut s = s.to_string();
        s.retain(|c| !c.is_whitespace());
        s.parse::<u64>().expect("should be a number")
    })(input)
}

#[tracing::instrument(skip(input))]
fn time_line(input: &str) -> IResult<&str, u64> {
    let (input, ts) = preceded(pair(tag("Time:"), space1), number)(input)?;

    info!(?ts);

    Ok((input, ts))
}

#[tracing::instrument(skip(input))]
fn distance_line(input: &str) -> IResult<&str, u64> {
    let (input, ds) = preceded(pair(tag("Distance:"), space1), number)(input)?;

    info!(?ds);

    Ok((input, ds))
}

#[tracing::instrument(skip(input))]
fn sheet(input: &str) -> IResult<&str, Sheet> {
    let (input, sheet) = map(
        separated_pair(time_line, line_ending, distance_line),
        |(time, distance)| Sheet { time, distance },
    )(input)?;

    info!(?sheet);

    Ok((input, sheet))
}

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    info!("processing input");

    let (_, sheet) = sheet(input)?;

    let time = sheet.time;
    let distance = sheet.distance;

    let result = (0..=time)
        .into_par_iter()
        .map(|n| n * (time - n))
        .filter(|d| *d > distance)
        .count();

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
    fn it_works() {
        let input = r##"Time:      7  15   30
Distance:  9  40  200
"##;
        let result = process(input).unwrap();
        assert_eq!(result, "71503");
    }
}
