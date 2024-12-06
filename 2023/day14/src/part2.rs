use anyhow::{Context, Result};
use nom::{
    character::complete::{line_ending, one_of},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use tracing::info;

#[derive(Debug, Hash, PartialEq, Clone)]
enum Rock {
    Round,
    Cube,
    Empty,
}

#[derive(Debug, Hash, PartialEq, Clone)]
struct Reflector {
    rocks: Vec<Vec<Rock>>,
}

#[derive(Debug)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Reflector {
    fn tilt_north(mut self) -> Self {
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

    fn tilt_west(mut self) -> Self {
        let mut rocks_move = true;

        let rows = self.rocks.len();
        let columns = self.rocks[0].len();

        while rocks_move {
            // Always stop moving unless we know rocks will move.
            rocks_move = false;

            for i in 0..rows {
                for j in 1..columns {
                    let rock = &self.rocks[i][j];

                    if rock != &Rock::Round {
                        continue;
                    }

                    if &self.rocks[i][j - 1] == &Rock::Empty {
                        rocks_move = true;

                        self.rocks[i][j - 1] = Rock::Round;
                        self.rocks[i][j] = Rock::Empty;
                    }
                }
            }
        }

        Reflector { rocks: self.rocks }
    }

    fn tilt_south(mut self) -> Self {
        let mut rocks_move = true;

        let rows = self.rocks.len();
        let columns = self.rocks[0].len();

        while rocks_move {
            // Always stop moving unless we know rocks will move.
            rocks_move = false;

            for i in (0..rows - 1).rev() {
                for j in 0..columns {
                    let rock = &self.rocks[i][j];

                    if rock != &Rock::Round {
                        continue;
                    }

                    if &self.rocks[i + 1][j] == &Rock::Empty {
                        rocks_move = true;

                        self.rocks[i + 1][j] = Rock::Round;
                        self.rocks[i][j] = Rock::Empty;
                    }
                }
            }
        }

        Reflector { rocks: self.rocks }
    }

    fn tilt_east(mut self) -> Self {
        let mut rocks_move = true;

        let rows = self.rocks.len();
        let columns = self.rocks[0].len();

        while rocks_move {
            // Always stop moving unless we know rocks will move.
            rocks_move = false;

            for i in 0..rows {
                for j in (0..columns - 1).rev() {
                    let rock = &self.rocks[i][j];

                    if rock != &Rock::Round {
                        continue;
                    }

                    if &self.rocks[i][j + 1] == &Rock::Empty {
                        rocks_move = true;

                        self.rocks[i][j + 1] = Rock::Round;
                        self.rocks[i][j] = Rock::Empty;
                    }
                }
            }
        }

        Reflector { rocks: self.rocks }
    }

    fn tilt(self, dir: Direction) -> Self {
        match dir {
            Direction::North => self.tilt_north(),
            Direction::West => self.tilt_west(),
            Direction::South => self.tilt_south(),
            Direction::East => self.tilt_east(),
        }
    }

    fn compute_load(self) -> usize {
        self.rocks
            .par_iter()
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

fn cycle(reflector: Reflector) -> Reflector {
    reflector
        .tilt(Direction::North)
        .tilt(Direction::West)
        .tilt(Direction::South)
        .tilt(Direction::East)
}

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    info!("processing input");

    let (_, mut reflector) = reflector(input)?;

    let mut history = vec![];

    let Some(cycle_start) = (0..1_000_000_000).find_map(|_| {
        history.push(reflector.clone());
        reflector = cycle(reflector.clone());
        history.par_iter().position_any(|prev| *prev == reflector)
    }) else {
        return Ok(reflector.compute_load().to_string());
    };

    let cycle_length = history.len() - cycle_start;
    let solution_index = cycle_start + ((1_000_000_000 - cycle_start) % cycle_length);
    let result = history[solution_index].clone().compute_load();

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
        assert_eq!(result, "64");
    }
}
