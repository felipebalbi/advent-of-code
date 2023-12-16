use anyhow::{Context, Result};
use std::collections::{HashSet, VecDeque};
use tracing::info;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Ray {
    direction: Direction,
    x: isize,
    y: isize,
}

impl Ray {
    fn reflect(&self, mirror: char) -> Direction {
        match mirror {
            '/' => match self.direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Right => Direction::Up,
                Direction::Left => Direction::Down,
            },
            '\\' => match self.direction {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Left => Direction::Up,
            },
            _ => unreachable!(),
        }
    }
}

fn out_of_bounds(grid: &Vec<Vec<char>>, pos: &(isize, isize)) -> bool {
    pos.0 < 0 || pos.0 >= (grid[0].len() as isize) || pos.1 < 0 || pos.1 >= (grid.len() as isize)
}

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    info!("processing input");

    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut next_position = VecDeque::<Ray>::new();

    // Starts at position 0 going Right
    next_position.push_back(Ray {
        direction: Direction::Right,
        x: 0,
        y: 0,
    });

    let mut visited = HashSet::<(isize, isize, Direction)>::new();
    visited.insert((0, 0, Direction::Right));

    while !next_position.is_empty() {
        let ray = next_position.pop_front().expect("should have a ray");

        let c = grid[ray.y as usize][ray.x as usize];

        match c {
            '.' => {
                let (x, y) = match ray.direction {
                    Direction::Right => (ray.x + 1, ray.y),
                    Direction::Left => (ray.x - 1, ray.y),
                    Direction::Down => (ray.x, ray.y + 1),
                    Direction::Up => (ray.x, ray.y - 1),
                };

                if !visited.contains(&(x, y, ray.direction)) && !out_of_bounds(&grid, &(x, y)) {
                    visited.insert((x, y, ray.direction));

                    next_position.push_back(Ray {
                        direction: ray.direction,
                        x,
                        y,
                    });
                }
            }
            '\\' => {
                let (x, y) = match ray.direction {
                    Direction::Up => (ray.x - 1, ray.y),
                    Direction::Down => (ray.x + 1, ray.y),
                    Direction::Left => (ray.x, ray.y - 1),
                    Direction::Right => (ray.x, ray.y + 1),
                };

                if !visited.contains(&(x, y, ray.reflect(c))) && !out_of_bounds(&grid, &(x, y)) {
                    visited.insert((x, y, ray.reflect(c)));

                    next_position.push_back(Ray {
                        direction: ray.reflect(c),
                        x,
                        y,
                    });
                }
            }
            '/' => {
                let (x, y) = match ray.direction {
                    Direction::Up => (ray.x + 1, ray.y),
                    Direction::Down => (ray.x - 1, ray.y),
                    Direction::Left => (ray.x, ray.y + 1),
                    Direction::Right => (ray.x, ray.y - 1),
                };

                if !visited.contains(&(x, y, ray.reflect(c))) && !out_of_bounds(&grid, &(x, y)) {
                    visited.insert((x, y, ray.reflect(c)));

                    next_position.push_back(Ray {
                        direction: ray.reflect(c),
                        x,
                        y,
                    });
                }
            }
            '-' => match ray.direction {
                Direction::Right => {
                    let (x, y) = (ray.x + 1, ray.y);

                    if !visited.contains(&(x, y, ray.direction)) && !out_of_bounds(&grid, &(x, y)) {
                        visited.insert((x, y, ray.direction));

                        next_position.push_back(Ray {
                            direction: ray.direction,
                            x,
                            y,
                        });
                    }
                }
                Direction::Left => {
                    let (x, y) = (ray.x - 1, ray.y);

                    if !visited.contains(&(x, y, ray.direction)) && !out_of_bounds(&grid, &(x, y)) {
                        visited.insert((x, y, ray.direction));

                        next_position.push_back(Ray {
                            direction: ray.direction,
                            x,
                            y,
                        });
                    }
                }
                Direction::Up | Direction::Down => {
                    let (x, y) = (ray.x - 1, ray.y);

                    if !visited.contains(&(x, y, Direction::Left)) && !out_of_bounds(&grid, &(x, y))
                    {
                        visited.insert((x, y, Direction::Left));

                        next_position.push_back(Ray {
                            direction: Direction::Left,
                            x,
                            y,
                        });
                    }

                    let (x, y) = (ray.x + 1, ray.y);

                    if !visited.contains(&(x, y, Direction::Right))
                        && !out_of_bounds(&grid, &(x, y))
                    {
                        visited.insert((x, y, Direction::Right));

                        next_position.push_back(Ray {
                            direction: Direction::Right,
                            x,
                            y,
                        });
                    }
                }
            },
            '|' => match ray.direction {
                Direction::Down => {
                    let (x, y) = (ray.x, ray.y + 1);

                    if !visited.contains(&(x, y, ray.direction)) && !out_of_bounds(&grid, &(x, y)) {
                        visited.insert((x, y, ray.direction));

                        next_position.push_back(Ray {
                            direction: ray.direction,
                            x,
                            y,
                        });
                    }
                }
                Direction::Up => {
                    let (x, y) = (ray.x, ray.y - 1);

                    if !visited.contains(&(x, y, ray.direction)) {
                        visited.insert((x, y, ray.direction));

                        next_position.push_back(Ray {
                            direction: ray.direction,
                            x,
                            y,
                        });
                    }
                }
                Direction::Right | Direction::Left => {
                    let (x, y) = (ray.x, ray.y - 1);

                    if !visited.contains(&(x, y, Direction::Up)) && !out_of_bounds(&grid, &(x, y)) {
                        visited.insert((x, y, Direction::Up));

                        next_position.push_back(Ray {
                            direction: Direction::Up,
                            x,
                            y,
                        });
                    }

                    let (x, y) = (ray.x, ray.y + 1);

                    if !visited.contains(&(x, y, Direction::Down)) && !out_of_bounds(&grid, &(x, y))
                    {
                        visited.insert((x, y, Direction::Down));

                        next_position.push_back(Ray {
                            direction: Direction::Down,
                            x,
                            y,
                        });
                    }
                }
            },
            '\n' => {}
            _ => unreachable!(),
        }
    }

    let energized_count = visited
        .iter()
        .map(|(x, y, _)| (x, y))
        .collect::<HashSet<(&isize, &isize)>>()
        .len();

    Ok(energized_count.to_string())
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
        let input = r##".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"##;
        let result = process(input).unwrap();
        assert_eq!(result, "46");
    }
}
