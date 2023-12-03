use anyhow::{Context, Result};
use glam::IVec2;
use nom::{
    branch::alt,
    bytes::complete::{is_not, take_till1},
    character::complete::digit1,
    combinator::iterator,
    IResult, Parser,
};
use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;
type SpanCoord<'a> = LocatedSpan<&'a str, IVec2>;

#[derive(Debug, PartialEq)]
enum Value<'a> {
    Empty,
    Symbol(SpanCoord<'a>),
    Number(SpanCoord<'a>),
}

#[tracing::instrument]
fn coord(input: Span) -> SpanCoord {
    let x = input.get_column() as i32 - 1;
    let y = input.location_line() as i32 - 1;
    input.map_extra(|_| IVec2::new(x, y))
}

#[tracing::instrument]
fn schematic(input: Span) -> IResult<Span, Vec<Value>> {
    let mut it = iterator(
        input,
        alt((
            digit1
                .map(|span| coord(span))
                .map(|digit| Value::Number(digit)),
            is_not(".\n0123456789")
                .map(|span| coord(span))
                .map(|s| Value::Symbol(s)),
            take_till1(|c: char| c.is_ascii_digit() || c != '.' && c != '\n').map(|_| Value::Empty),
        )),
    );

    let parsed = it
        .filter(|value| value != &Value::Empty)
        .collect::<Vec<Value>>();
    let res: IResult<_, _> = it.finish();

    res.map(|(input, _)| (input, parsed))
}

#[tracing::instrument]
fn process(input: &'static str) -> Result<String> {
    let (_, schematic) = schematic(Span::new(input))?;

    let sum = schematic
        .iter()
        .filter_map(|value| {
            let Value::Number(num) = value else {
                return None;
            };
            let neighbors = [
                // east border
                IVec2::new(num.fragment().len() as i32, 0),
                // west border
                IVec2::new(-1, 0),
            ]
            .into_iter()
            .chain(
                // north border
                (-1..=num.fragment().len() as i32).map(|x_offset| IVec2::new(x_offset, 1)),
            )
            .chain(
                // south border
                (-1..=num.fragment().len() as i32).map(|x_offset| IVec2::new(x_offset, -1)),
            )
            .map(|pos| pos + num.extra)
            .collect::<Vec<IVec2>>();

            schematic
                .iter()
                .any(|symbol| {
                    let Value::Symbol(sym) = symbol else {
                        return false;
                    };
                    neighbors.iter().find(|pos| pos == &&sym.extra).is_some()
                })
                .then_some(
                    num.fragment()
                        .parse::<u32>()
                        .expect("should be a valid number"),
                )
        })
        .sum::<u32>();

    Ok(sum.to_string())
}

#[tracing::instrument]
pub fn part1() -> Result<()> {
    let file = include_str!("../input1.txt");
    let result = process(file).context("process part 1")?;
    println!("Part 1: {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = r##"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"##;
        let result = process(input).unwrap();
        assert_eq!(result, "4361");
    }
}
