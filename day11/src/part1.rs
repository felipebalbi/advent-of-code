use anyhow::{Context, Result};
use glam::IVec2;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    multi::{many1, separated_list1},
    IResult, Parser,
};
use nom_locate::LocatedSpan;
use tracing::info;

type Span<'a> = LocatedSpan<&'a str>;
type SpanCoord<'a> = LocatedSpan<&'a str, IVec2>;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Value<'a> {
    Empty(SpanCoord<'a>),
    Galaxy(SpanCoord<'a>),
}

impl<'a> Value<'a> {
    fn coordinates(&self) -> IVec2 {
        match self {
            Value::Galaxy(coord) => coord.extra,
            _ => IVec2::new(0, 0),
        }
    }
}

#[tracing::instrument(skip(input))]
fn coordinate(input: Span) -> SpanCoord {
    let x = input.get_column() as i32 - 1;
    let y = input.location_line() as i32 - 1;
    input.map_extra(|_| IVec2::new(x, y))
}

#[tracing::instrument(skip(input))]
fn line(input: Span) -> IResult<Span, Vec<Value>> {
    many1(alt((
        tag(".")
            .map(|span| coordinate(span))
            .map(|empty| Value::Empty(empty)),
        tag("#")
            .map(|span| coordinate(span))
            .map(|pound| Value::Galaxy(pound)),
    )))(input)
}

#[tracing::instrument(skip(input))]
fn galaxies(input: Span) -> IResult<Span, Vec<Vec<Value>>> {
    separated_list1(line_ending, line)(input)
}

#[tracing::instrument(skip(v))]
fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<&T>> {
    assert!(!v.is_empty());

    (0..v[0].len())
        .map(|i| v.iter().map(|inner| &inner[i]).collect::<Vec<&T>>())
        .collect()
}

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    info!("processing input");

    let (_, galaxies) = galaxies(Span::new(input))?;

    let empty_rows = galaxies
        .iter()
        .filter(|row| {
            row.iter().all(|value| match value {
                Value::Empty(_) => true,
                _ => false,
            })
        })
        .map(|row| {
            let first = row.first().expect("row should have at least one member");

            match first {
                Value::Empty(coordinate) => coordinate.extra.y,
                _ => unreachable!(),
            }
        })
        .unique()
        .collect::<Vec<_>>();

    let transposed = transpose(&galaxies);
    let empty_cols = transposed
        .iter()
        .filter(|col| {
            col.iter().all(|value| match value {
                Value::Empty(_) => true,
                _ => false,
            })
        })
        .map(|col| {
            let first = col.first().expect("col should have at least one member");

            match first {
                Value::Empty(coordinate) => coordinate.extra.x,
                _ => unreachable!(),
            }
        })
        .unique()
        .collect::<Vec<_>>();

    let result = galaxies
        .iter()
        .flatten()
        .filter(|value| match value {
            Value::Galaxy(_) => true,
            _ => false,
        })
        .tuple_combinations()
        .map(|(a, b)| {
            let mut coord1 = a.coordinates();
            let mut coord2 = b.coordinates();

            for empty_col in empty_cols.iter() {
                if a.coordinates().x > *empty_col {
                    coord1 += IVec2::new(1, 0);
                }

                if b.coordinates().x > *empty_col {
                    coord2 += IVec2::new(1, 0);
                }
            }

            for empty_row in empty_rows.iter() {
                if a.coordinates().y > *empty_row {
                    coord1 += IVec2::new(0, 1);
                }

                if b.coordinates().y > *empty_row {
                    coord2 += IVec2::new(0, 1);
                }
            }

            (coord1.x - coord2.x).abs() + (coord1.y - coord2.y).abs()
        })
        .collect::<Vec<_>>();

    let sum = result.iter().sum::<i32>();

    Ok(sum.to_string())
}

#[tracing::instrument(skip(input))]
pub fn part1(input: &'static str) -> Result<String> {
    info!("part 1");

    process(&input).context("process part 1")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn it_works() {
        let input = r##"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"##;
        let result = process(input).unwrap();
        assert_eq!(result, "374");
    }
}
