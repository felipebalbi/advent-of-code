use anyhow::{Context, Result};
use pathfinding::{
    directed::dijkstra::dijkstra_reach,
    matrix::{directions, Matrix},
};
use tracing::info;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Tile {
    Start,
    Garden,
    Rock,
}

const STEPS_AMOUNT: usize = 64 - 1;

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    info!("processing input");

    let grid = input
        .lines()
        .map(|line| {
            line.chars().map(|c| match c {
                '.' => Tile::Garden,
                '#' => Tile::Rock,
                'S' => Tile::Start,
                _ => unreachable!("invalid input"),
            })
        })
        .collect::<Matrix<Tile>>();

    let start_position = grid
        .items()
        .find(|(_, tile)| *tile == &Tile::Start)
        .map(|(pos, _)| pos)
        .expect("should have a start position");

    let result = dijkstra_reach(&start_position, |&pos, cost| {
        let successors = directions::DIRECTIONS_4
            .into_iter()
            .filter_map(|direction| {
                if let Some(neighbor_position) = grid.move_in_direction(pos, direction) {
                    let tile = *grid.get(neighbor_position).expect("should have a neighbor");

                    if tile == Tile::Rock || cost > STEPS_AMOUNT {
                        None
                    } else {
                        Some((neighbor_position, 1))
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        successors
    })
    .filter(|garden| garden.total_cost % 2 == 0)
    .count();

    Ok(result.to_string())
}

#[tracing::instrument(skip(input))]
pub fn part1(input: &'static str) -> Result<String> {
    info!("part 1");

    process(input).context("process part 1")
}
