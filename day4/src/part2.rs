use anyhow::{Context, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space0, space1},
    combinator::map,
    multi::{fold_many1, separated_list1},
    sequence::{pair, tuple},
    IResult,
};
use std::collections::{BTreeMap, HashSet};

#[derive(Debug)]
struct Card {
    id: u32,
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
        |(_, _, id, _, _, winning, _, _, own)| Card { id, winning, own },
    )(input)
}

#[tracing::instrument]
fn cards(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(line_ending, card)(input)
}

#[tracing::instrument]
fn process(input: &'static str) -> Result<String> {
    let (_, cards) = cards(input)?;

    let mut map = BTreeMap::<u32, u32>::new();

    let _ = cards
        .iter()
        .map(|card| {
            map.entry(card.id).and_modify(|e| *e += 1).or_insert(1);

            let count = card.winning.intersection(&card.own).count();

            let times = *map.get(&card.id).unwrap();

            for i in 1..=count {
                map.entry(card.id + i as u32)
                    .and_modify(|e| *e += times)
                    .or_insert(times);
            }

            card
        })
        .count();

    let amount = map.values().sum::<u32>();

    Ok(amount.to_string())
}

#[tracing::instrument]
pub fn part2() -> Result<()> {
    let file = include_str!("../input1.txt");
    let result = process(file).context("process part 2")?;
    println!("Part 2: {}", result);

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
        assert_eq!(result, "30");
    }
}
