use anyhow::{Context, Result};
use glam::I64Vec2;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{self, line_ending, one_of, space1},
    combinator::{map, value},
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};
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
    amount: i64,
}

fn color(input: &str) -> IResult<&str, ()> {
    value((), take_until(")"))(input)
}

#[tracing::instrument(skip(input))]
fn amount(input: &str) -> IResult<&str, i64> {
    complete::i64(input)
}

#[tracing::instrument(skip(input))]
fn direction(input: &str) -> IResult<&str, Direction> {
    map(one_of("UDLR"), |c| match c {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => unreachable!(),
    })(input)
}

#[tracing::instrument(skip(input))]
fn instruction(input: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            direction,
            space1,
            amount,
            space1,
            delimited(tag("("), color, tag(")")),
        )),
        |(direction, _, amount, _, _)| Instruction { direction, amount },
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

    let (inner_area, perimeter, _) = dig_plan.iter().fold(
        (0, 1, I64Vec2::splat(0)),
        |(area, perimeter, current), inst| {
            let delta = match inst.direction {
                Direction::Up => I64Vec2::new(0, -inst.amount),
                Direction::Down => I64Vec2::new(0, inst.amount),
                Direction::Left => I64Vec2::new(-inst.amount, 0),
                Direction::Right => I64Vec2::new(inst.amount, 0),
            };

            (
                area + delta.x * current.y,
                perimeter + delta.x.abs() + delta.y.abs(),
                current + delta,
            )
        },
    );

    let area = inner_area.abs() + perimeter / 2 + 1;

    Ok(area.to_string())
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
        assert_eq!(result, "62");
    }
}
