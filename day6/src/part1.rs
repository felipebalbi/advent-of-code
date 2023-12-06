use anyhow::{Context, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{pair, preceded, separated_pair},
    IResult,
};
use tracing::info;

#[derive(Debug)]
struct Sheet {
    times: Vec<u32>,
    distances: Vec<u32>,
}

#[tracing::instrument(skip(input))]
fn numbers(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(space1, complete::u32)(input)
}

#[tracing::instrument(skip(input))]
fn time_line(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, ts) = preceded(pair(tag("Time:"), space1), numbers)(input)?;

    info!(?ts);

    Ok((input, ts))
}

#[tracing::instrument(skip(input))]
fn distance_line(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, ds) = preceded(pair(tag("Distance:"), space1), numbers)(input)?;

    info!(?ds);

    Ok((input, ds))
}

#[tracing::instrument(skip(input))]
fn sheet(input: &str) -> IResult<&str, Sheet> {
    let (input, sheet) = map(
        separated_pair(time_line, line_ending, distance_line),
        |(times, distances)| Sheet { times, distances },
    )(input)?;

    info!(?sheet);

    Ok((input, sheet))
}

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    info!("processing input");

    let (_, sheet) = sheet(input)?;

    let result = sheet
        .times
        .into_iter()
        .zip(sheet.distances)
        .map(|(time, distance)| {
            let time = time as f64;
            let distance = distance as f64;

            let determinant = (time * time - 4.0 * distance).sqrt();
            let r1 = (time + determinant) / 2.0;
            let r2 = (time - determinant) / 2.0;
            let max = r1.max(r2).ceil();
            let min = r1.min(r2).floor();

            let max = max - 1.0;

            info!(?time, ?distance, ?r1, ?r2, ?max, ?min);

            (max - min) as u32
        })
        .product::<u32>();

    info!(?result);

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
        let input = r##"Time:      7  15   30
Distance:  9  40  200
"##;
        let result = process(input).unwrap();
        assert_eq!(result, "288");
    }
}
