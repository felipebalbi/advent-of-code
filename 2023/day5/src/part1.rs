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
use std::ops::Range;
use tracing::info;

#[derive(Debug, PartialEq)]
struct Map<'a> {
    name: &'a str,
    ranges: Vec<(Range<u64>, Range<u64>)>,
}

#[derive(Debug)]
struct Almanac<'a> {
    seeds: Vec<u64>,
    maps: Vec<Map<'a>>,
}

#[tracing::instrument(skip(input))]
fn number(input: &str) -> IResult<&str, u64> {
    map(pair(complete::u64, space0), |(n, _)| n)(input)
}

#[tracing::instrument(skip(input))]
fn numbers(input: &str) -> IResult<&str, Vec<u64>> {
    many1(number)(input)
}

#[tracing::instrument(skip(input))]
fn seeds(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, seeds) = map(
        terminated(
            tuple((tag("seeds:"), space1, numbers)),
            pair(line_ending, line_ending),
        ),
        |(_, _, ns)| ns,
    )(input)?;

    info!(?seeds);

    Ok((input, seeds))
}

#[tracing::instrument(skip(input))]
fn map_name(input: &str) -> IResult<&str, &str> {
    recognize(many1(alt((alpha1, tag("-")))))(input)
}

#[tracing::instrument(skip(input))]
fn range(input: &str) -> IResult<&str, Vec<u64>> {
    numbers(input)
}

#[tracing::instrument(skip(input))]
fn almanac_map(input: &str) -> IResult<&str, Map> {
    map(
        tuple((
            map_name,
            space1,
            tag("map:"),
            line_ending,
            separated_list1(line_ending, range),
        )),
        |(name, _, _, _, ranges)| {
            let ranges = ranges
                .into_iter()
                .map(|range| {
                    let dst = range[0];
                    let src = range[1];
                    let len = range[2];

                    ((src..src + len), (dst..dst + len))
                })
                .collect::<Vec<_>>();

            let map = Map { name, ranges };

            info!(?map);

            map
        },
    )(input)
}

#[tracing::instrument(skip(input))]
fn almanac_maps(input: &str) -> IResult<&str, Vec<Map>> {
    separated_list1(pair(line_ending, line_ending), almanac_map)(input)
}

#[tracing::instrument(skip(input))]
fn almanac(input: &str) -> IResult<&str, Almanac> {
    map(pair(seeds, almanac_maps), |(seeds, maps)| Almanac {
        seeds,
        maps,
    })(input)
}

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    let (_, almanac) = almanac(input)?;

    let closest = almanac
        .seeds
        .iter()
        .map(|seed| {
            almanac.maps.iter().fold(*seed, |seed, map| {
                let valid = map
                    .ranges
                    .iter()
                    .find(|(source_range, _)| source_range.contains(&seed));

                let Some((source, destination)) = valid else {
                    return seed;
                };

                let offset = seed - source.start;
                destination.start + offset
            })
        })
        .min()
        .unwrap();

    Ok(closest.to_string())
}

#[tracing::instrument(skip(input))]
pub fn part1(input: &'static str) -> Result<String> {
    process(input).context("process part 1")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn it_works() {
        let input = dbg!(
            r##"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"##
        );
        let result = process(input).unwrap();
        assert_eq!(result, "35");
    }
}
