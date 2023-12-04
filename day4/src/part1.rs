use anyhow::{Context, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space0, space1},
    combinator::map,
    multi::{fold_many1, separated_list1},
    sequence::{pair, tuple},
    IResult,
};
use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    winning: HashSet<u32>,
    own: HashSet<u32>,
}

#[tracing::instrument]
fn numbers(input: &str) -> IResult<&str, HashSet<u32>> {
    fold_many1(
        pair(complete::u32, space0),
        HashSet::new,
        |mut acc, (number, _)| {
            acc.insert(number);
            acc
        },
    )(input)
}

#[tracing::instrument]
fn own_numbers(input: &str) -> IResult<&str, HashSet<u32>> {
    numbers(input)
}

#[tracing::instrument]
fn winning_numbers(input: &str) -> IResult<&str, HashSet<u32>> {
    numbers(input)
}

#[tracing::instrument]
fn card(input: &str) -> IResult<&str, Card> {
    map(
        tuple((
            tag("Card"),
            space1,
            complete::u32,
            tag(":"),
            space1,
            winning_numbers,
            tag("|"),
            space1,
            own_numbers,
        )),
        |(_, _, _, _, _, winning, _, _, own)| Card { winning, own },
    )(input)
}

#[tracing::instrument]
fn cards(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(line_ending, card)(input)
}

#[tracing::instrument]
fn process(input: &'static str) -> Result<String> {
    let (_, cards) = cards(input)?;

    let points = cards
        .iter()
        .map(|card| {
            let power = card.winning.intersection(&card.own).count();

            if power == 0 {
                0
            } else {
                2_usize.pow((power - 1) as u32)
            }
        })
        .sum::<usize>();

    Ok(points.to_string())
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
        let input = r##"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"##;
        let result = process(input).unwrap();
        assert_eq!(result, "13");
    }
}
