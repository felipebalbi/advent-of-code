use anyhow::{Context, Result};
use nom::{
    character::complete::{line_ending, one_of},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};
use tracing::info;

#[derive(Debug, PartialEq)]
enum Rock {
    Round,
    Cube,
    Empty,
}

#[derive(Debug)]
struct Reflector {
    rocks: Vec<Vec<Rock>>,
}

impl Reflector {
    fn tilt(mut self) -> Self {
        let mut rocks_move = true;

        let rows = self.rocks.len();
        let columns = self.rocks[0].len();

        while rocks_move {
            // Always stop moving unless we know rocks will move.
            rocks_move = false;

            for i in 1..rows {
                for j in 0..columns {
                    let rock = &self.rocks[i][j];

                    if rock != &Rock::Round {
                        continue;
                    }

                    // If the rock above us is an empty space, move this rock there
                    if &self.rocks[i - 1][j] == &Rock::Empty {
                        rocks_move = true;

                        self.rocks[i - 1][j] = Rock::Round;
                        self.rocks[i][j] = Rock::Empty;
                    }
                }
            }
        }

        Reflector { rocks: self.rocks }
    }

    fn compute_load(self) -> usize {
        self.rocks
            .iter()
            .rev()
            .enumerate()
            .map(|(i, row)| (i + 1) * row.iter().filter(|rock| *rock == &Rock::Round).count())
            .sum::<usize>()
    }
}

#[tracing::instrument(skip(input))]
fn rock(input: &str) -> IResult<&str, Rock> {
    map(one_of("O#."), |c| match c {
        'O' => Rock::Round,
        '#' => Rock::Cube,
        '.' => Rock::Empty,
        _ => unreachable!(),
    })(input)
}

#[tracing::instrument(skip(input))]
fn rocks(input: &str) -> IResult<&str, Vec<Rock>> {
    many1(rock)(input)
}

#[tracing::instrument(skip(input))]
fn reflector(input: &str) -> IResult<&str, Reflector> {
    map(separated_list1(line_ending, rocks), |rocks| Reflector {
        rocks,
    })(input)
}

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    info!("processing input");

    let (_, reflector) = reflector(input)?;

    info!(?reflector);

    let result = reflector.tilt().compute_load();

    Ok(result.to_string())
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
        let input = r##"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"##;
        let result = process(input).unwrap();
        assert_eq!(result, "136");
    }
}
