use anyhow::{Context, Result};
use itertools::Itertools;
use nom::{
    character::complete::{self, line_ending, one_of, space1},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};
use std::collections::BTreeMap;
use tracing::info;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    J,
    Value(u32),
    T,
    Q,
    K,
    A,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            c if c.is_digit(10) => Self::Value(c.to_digit(10).expect("should be a number")),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum RankedHand {
    HighCard(Vec<Card>),
    OnePair(Vec<Card>),
    TwoPairs(Vec<Card>),
    ThreeOfAKind(Vec<Card>),
    FullHouse(Vec<Card>),
    FourOfAKind(Vec<Card>),
    FiveOfAKind(Vec<Card>),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    cards: RankedHand,
    bid: u32,
}

#[tracing::instrument(skip(input))]
fn bid(input: &str) -> IResult<&str, u32> {
    complete::u32(input)
}

#[tracing::instrument(skip(input))]
fn card(input: &str) -> IResult<&str, Card> {
    map(one_of("AKQJT98765432"), |c| c.into())(input)
}

#[tracing::instrument(skip(input))]
fn cards(input: &str) -> IResult<&str, RankedHand> {
    map(many1(card), |cards| {
        let mut map = cards
            .iter()
            .cloned()
            .fold(BTreeMap::<Card, u32>::new(), |mut acc, c| {
                acc.entry(c).and_modify(|e| *e += 1).or_insert(1);
                acc
            });

        if map.keys().contains(&Card::J) {
            let jokers = map.get(&Card::J).expect("should have a joker").clone();

            let mut best = map.iter().next_back().unwrap().clone();

            for (key, value) in map.iter() {
                if value > best.1 && key != &Card::J {
                    best = (key, value);
                }
            }

            if best.0 != &Card::J {
                map.entry(best.0.clone()).and_modify(|e| *e += jokers);
            }
        }

        let mut no_jokers = map
            .into_iter()
            .filter(|(card, _)| card != &Card::J)
            .collect::<BTreeMap<Card, u32>>();

        // If we end up with an empty map, it can only mean we had a
        // hand of five jokers.
        if no_jokers.is_empty() {
            no_jokers.insert(Card::J, 5);
        }

        let ranked_hand = match no_jokers.values().len() {
            5 => RankedHand::HighCard(cards),
            4 => RankedHand::OnePair(cards),
            3 if no_jokers.values().contains(&&3) => RankedHand::ThreeOfAKind(cards),
            3 if no_jokers.values().contains(&&2) => RankedHand::TwoPairs(cards),
            2 if no_jokers.values().contains(&&4) => RankedHand::FourOfAKind(cards),
            2 if no_jokers.values().contains(&&2) && no_jokers.values().contains(&&3) => {
                RankedHand::FullHouse(cards)
            }
            1 => RankedHand::FiveOfAKind(cards),
            _ => unreachable!(),
        };

        info!(?ranked_hand);

        ranked_hand
    })(input)
}

#[tracing::instrument(skip(input))]
fn hand(input: &str) -> IResult<&str, Hand> {
    map(separated_pair(cards, space1, bid), |(cards, bid)| Hand {
        cards,
        bid,
    })(input)
}

#[tracing::instrument(skip(input))]
fn hands(input: &str) -> IResult<&str, Vec<Hand>> {
    separated_list1(line_ending, hand)(input)
}

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    info!("processing input");

    let (_, mut hands) = hands(input)?;
    hands.sort();

    let result = hands
        .iter()
        .enumerate()
        .map(|(index, hand)| (index + 1) as u32 * hand.bid)
        .sum::<u32>();

    Ok(result.to_string())
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
        let input = r##"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"##;
        let result = process(input).unwrap();
        assert_eq!(result, "5905");
    }
}
