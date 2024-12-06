use anyhow::{Context, Result};
use pathfinding::{
    directed::dijkstra::dijkstra,
    matrix::{directions, Matrix},
};
use tracing::info;

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    info!("processing input");

    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("should be a valid number"))
        })
        .collect::<Matrix<u32>>();

    let path = dijkstra(
        &((0, 0), directions::E, 0),
        |&(pos, step_direction, same_direction_count)| {
            directions::DIRECTIONS_4
                .into_iter()
                .filter_map(|direction| {
                    grid.move_in_direction(pos, direction)
                        .map(|neighbor_position| {
                            // add a large penalty if we attempt to travel less than 4 steps in the
                            // same direction
                            let cost = if same_direction_count < 4 && direction != step_direction {
                                420
                            } else {
                                *grid.get(neighbor_position).unwrap() as usize
                            };

                            // info!(?direction, ?step_direction, ?same_direction_count);

                            if direction != (step_direction.0 * -1, step_direction.1 * -1)
                                && direction != step_direction
                            {
                                // changing direction, reset same direction count to 1
                                ((neighbor_position, direction, 1), cost)
                            } else if same_direction_count < 10 && direction == step_direction {
                                // same direction, increment same direction count
                                (
                                    (neighbor_position, direction, same_direction_count + 1),
                                    cost,
                                )
                            } else {
                                // Add a node that we will never follow
                                ((neighbor_position, direction, 3), 420)
                            }
                        })
                })
                .collect::<Vec<_>>()
        },
        |&(pos, _, dist)| pos == (grid.rows - 1, grid.columns - 1) && dist >= 4,
    )
    .expect("should have found a path");

    info!(?path);

    let total_cost = path.1;
    Ok(total_cost.to_string())
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
    fn small_example() {
        let input = r##"111111111111
999999999991
999999999991
999999999991
999999999991
"##;
        let result = process(input).unwrap();
        assert_eq!(result, "71");
    }

    #[test_log::test]
    fn it_works() {
        let input = r##"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"##;
        let result = process(input).unwrap();
        assert_eq!(result, "94");
    }
}
