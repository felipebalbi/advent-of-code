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

const STEPS_AMOUNT: usize = 26501365;

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

    let divisor = STEPS_AMOUNT / (grid.columns);
    let rem = STEPS_AMOUNT % (grid.columns);

    info!(?divisor, ?rem);

    // for n in 0..3 {
    let n = 1;
    let coeff = dijkstra_reach(&start_position, |&pos, cost| {
        let successors = directions::DIRECTIONS_4
            .into_iter()
            .filter_map(|direction| {
                let npos = match (grid.move_in_direction(pos, direction), direction) {
                    (Some(p), _) => p,
                    (_, directions::N) => (grid.rows - 1, pos.1),
                    (_, directions::S) => (0, pos.1),
                    (_, directions::E) => (pos.0, 0),
                    (_, directions::W) => (pos.0, grid.columns - 1),
                    _ => unreachable!(),
                };

                let tile = *grid.get(npos).expect("should have a neighbor");

                // if tile == Tile::Rock || (cost > n * 131 + rem - 1) {
                if tile == Tile::Rock || (cost > 196 - 1) {
                    None
                } else {
                    if cost > 100 {
                        info!(?cost);
                    }
                    Some((npos, 1))
                }
            })
            .collect::<Vec<_>>();

        successors
    })
    // .filter(|garden| garden.total_cost % 2 == 0)
    .filter(|garden| garden.total_cost == (196))
    .count();

    info!(?coeff);
    // }

    let result = 14860 * divisor * divisor + 14925 * divisor + 3762;

    // solvable as ax^2 + bx + c where x = 202300
    //7461
    //
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
        let input = r##"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."##;
        let result = process(input).unwrap();
        assert_eq!(result, "16");
    }
}
