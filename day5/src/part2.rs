use anyhow::{Context, Result};
use indicatif::ParallelProgressIterator;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, space0, space1},
    combinator::{map, recognize},
    multi::{many1, separated_list1},
    sequence::{pair, terminated, tuple},
    IResult,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::ops::Range;

#[derive(Debug, PartialEq)]
struct Map<'a> {
    name: &'a str,
    ranges: Vec<(Range<u64>, Range<u64>)>,
}

#[derive(Debug)]
struct Almanac<'a> {
    seeds: Vec<Range<u64>>,
    maps: Vec<Map<'a>>,
}

#[tracing::instrument]
fn number(input: &str) -> IResult<&str, u64> {
    map(pair(complete::u64, space0), |(n, _)| n)(input)
}

#[tracing::instrument]
fn numbers(input: &str) -> IResult<&str, Vec<u64>> {
    many1(number)(input)
}

#[tracing::instrument]
fn seed_range(input: &str) -> IResult<&str, Range<u64>> {
    map(pair(number, number), |(start, len)| (start..(start + len)))(input)
}

#[tracing::instrument]
fn seed_ranges(input: &str) -> IResult<&str, Vec<Range<u64>>> {
    many1(seed_range)(input)
}

#[tracing::instrument]
fn seeds(input: &str) -> IResult<&str, Vec<Range<u64>>> {
    map(
        terminated(
            tuple((tag("seeds:"), space1, seed_ranges)),
            pair(line_ending, line_ending),
        ),
        |(_, _, ns)| ns,
    )(input)
}

#[tracing::instrument]
fn map_name(input: &str) -> IResult<&str, &str> {
    recognize(many1(alt((alpha1, tag("-")))))(input)
}

#[tracing::instrument]
fn range(input: &str) -> IResult<&str, Vec<u64>> {
    numbers(input)
}

#[tracing::instrument]
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

            Map { name, ranges }
        },
    )(input)
}

#[tracing::instrument]
fn almanac_maps(input: &str) -> IResult<&str, Vec<Map>> {
    separated_list1(pair(line_ending, line_ending), almanac_map)(input)
}

#[tracing::instrument]
fn almanac(input: &str) -> IResult<&str, Almanac> {
    map(pair(seeds, almanac_maps), |(seeds, maps)| Almanac {
        seeds,
        maps,
    })(input)
}

#[tracing::instrument]
fn process(input: &'static str) -> Result<String> {
    let (_, almanac) = almanac(input)?;

    let locations = almanac
        .seeds
        .iter()
        .flat_map(|range| range.clone())
        .collect::<Vec<u64>>();

    let closest = locations
        .into_par_iter()
        .progress()
        .map(|seed| {
            almanac.maps.iter().fold(seed, |seed, map| {
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

#[tracing::instrument]
pub fn part2(input: &'static str) -> Result<String> {
    process(input).context("process part 2")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = r##"seeds: 79 14 55 13

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
"##;
        let result = process(input).unwrap();
        assert_eq!(result, "46");
    }
}
