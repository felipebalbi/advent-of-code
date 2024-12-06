use anyhow::{Context, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, newline, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{pair, separated_pair, terminated, tuple},
    IResult,
};
use std::cmp::max;

#[derive(Debug, PartialEq, PartialOrd)]
enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}

#[derive(Debug)]
struct Game {
    subsets: Vec<(Cube, Cube, Cube)>,
}

#[tracing::instrument]
fn cube(input: &str) -> IResult<&str, Cube> {
    map(
        separated_pair(complete::u32, space1, alpha1),
        |(quantity, color)| match color {
            "red" => Cube::Red(quantity),
            "green" => Cube::Green(quantity),
            "blue" => Cube::Blue(quantity),
            _ => unreachable!(),
        },
    )(input)
}

#[tracing::instrument]
fn subset(input: &str) -> IResult<&str, (Cube, Cube, Cube)> {
    let mut subset = (Cube::Red(0), Cube::Green(0), Cube::Blue(0));
    let (input, cubes) = separated_list1(terminated(tag(","), space1), cube)(input)?;

    for c in cubes {
        match c {
            Cube::Red(_) => subset.0 = c,
            Cube::Green(_) => subset.1 = c,
            Cube::Blue(_) => subset.2 = c,
        }
    }

    Ok((input, subset))
}

#[tracing::instrument]
fn game(input: &str) -> IResult<&str, Game> {
    map(
        pair(
            map(
                tuple((tag("Game "), complete::u32, tag(": "))),
                |(_, _, _)| (),
            ),
            separated_list1(tag("; "), subset),
        ),
        |(_, subsets)| Game { subsets },
    )(input)
}

#[tracing::instrument]
fn games(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(newline, game)(input)
}

#[tracing::instrument]
fn process(input: &'static str) -> Result<String> {
    let (_, games) = games(input)?;

    let sum = games
        .iter()
        .map(|game| {
            let (i, j, k) = game
                .subsets
                .iter()
                .map(|(r, g, b)| match (r, g, b) {
                    (Cube::Red(i), Cube::Green(j), Cube::Blue(k)) => (i, j, k),
                    _ => unreachable!(),
                })
                .fold((0, 0, 0), |(a, b, c), (i, j, k)| {
                    (max(a, *i), max(b, *j), max(c, *k))
                });

            i * j * k
        })
        .sum::<u32>();

    Ok(sum.to_string())
}

#[tracing::instrument]
pub fn part2() -> Result<()> {
    let file = include_str!("../input2.txt");
    let result = process(file).context("process part 2")?;
    println!("Part 2: {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = r##"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"##;
        let result = process(input).unwrap();
        assert_eq!(result, "2286");
    }
}
