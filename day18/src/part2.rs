use anyhow::{Context, Result};
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_until, take_while_m_n},
    character::complete::{line_ending, one_of},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use polygonical::{point::Point, polygon::Polygon};
use tracing::info;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    amount: u32,
}

#[tracing::instrument(skip(c))]
fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

#[tracing::instrument(skip(input))]
fn amount(input: &str) -> IResult<&str, u32> {
    map_res(take_while_m_n(5, 5, is_hex_digit), |input| {
        u32::from_str_radix(input, 16)
    })(input)
}

#[tracing::instrument(skip(input))]
fn direction(input: &str) -> IResult<&str, Direction> {
    map(one_of("0123"), |c| match c {
        '0' => Direction::Right,
        '1' => Direction::Down,
        '2' => Direction::Left,
        '3' => Direction::Up,
        _ => unreachable!(),
    })(input)
}

#[tracing::instrument(skip(input))]
fn instruction(input: &str) -> IResult<&str, Instruction> {
    map(
        tuple((take_until("#"), tag("#"), amount, direction, tag(")"))),
        |(_, _, amount, direction, _)| Instruction { direction, amount },
    )(input)
}

#[tracing::instrument(skip(input))]
fn dig_plan(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, instruction)(input)
}

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    info!("processing input");

    let (_, dig_plan) = dig_plan(input)?;

    info!(?dig_plan);

    let points = dig_plan
        .iter()
        .scan(Point::new(0.0, 0.0), |state, inst| {
            match inst.direction {
                Direction::Up => state.y -= inst.amount as f64,
                Direction::Down => state.y += inst.amount as f64,
                Direction::Left => state.x -= inst.amount as f64,
                Direction::Right => state.x += inst.amount as f64,
            };

            Some(*state)
        })
        .collect::<Vec<_>>();

    let perimeter = points
        .iter()
        .circular_tuple_windows()
        .fold(0.0, |acc, (p1, p2)| {
            acc + (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
        });

    let poly = Polygon::new(points);
    let area = poly.area().abs() + perimeter / 2.0 + 1.0;

    Ok(area.to_string())
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
        let input = r##"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
"##;
        let result = process(input).unwrap();
        assert_eq!(result, "952408144115");
    }
}
