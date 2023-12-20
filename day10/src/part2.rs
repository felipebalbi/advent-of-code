use anyhow::{Context, Result};
use pathfinding::matrix::{directions, Matrix};
use tracing::info;

#[tracing::instrument(skip(input))]
fn points(input: &str) -> Vec<(usize, usize)> {
    let grid = input
        .lines()
        .map(|line| line.chars())
        .collect::<Matrix<char>>();

    let start_coord = grid
        .items()
        .find(|(_, c)| *c == &'S')
        .map(|((row, col), _)| (row, col))
        .expect("should have a start");

    info!(?start_coord);

    let mut points = vec![start_coord];

    // find all valid directions from the S node
    let valid_neighbors = directions::DIRECTIONS_4
        .into_iter()
        .filter_map(|direction| {
            grid.move_in_direction((start_coord.0, start_coord.1), direction)
                .filter(|neighbor_position| match direction {
                    directions::N => {
                        if let Some(c) = grid.get(*neighbor_position) {
                            match *c {
                                '|' | 'F' | '7' => true,
                                _ => false,
                            }
                        } else {
                            false
                        }
                    }
                    directions::S => {
                        if let Some(c) = grid.get(*neighbor_position) {
                            match *c {
                                '|' | 'L' | 'J' => true,
                                _ => false,
                            }
                        } else {
                            false
                        }
                    }
                    directions::E => {
                        if let Some(c) = grid.get(*neighbor_position) {
                            match *c {
                                '-' | 'J' | '7' => true,
                                _ => false,
                            }
                        } else {
                            false
                        }
                    }
                    directions::W => {
                        if let Some(c) = grid.get(*neighbor_position) {
                            match *c {
                                '-' | 'L' | 'F' => true,
                                _ => false,
                            }
                        } else {
                            false
                        }
                    }
                    _ => false,
                })
        })
        .collect::<Vec<_>>();

    // Take the first and find a matching corner
    let first_neighbor = valid_neighbors
        .first()
        .expect("should have a first element");

    let mut next_point = start_coord;

    let mut travel_direction = (
        first_neighbor.0 as isize - start_coord.0 as isize,
        first_neighbor.1 as isize - start_coord.1 as isize,
    );

    while let Some(p) = grid.move_in_direction(next_point, travel_direction) {
        if let Some(c) = grid.get(p) {
            if *c == 'S' {
                break;
            }

            next_point = p;
            points.push(p);

            match travel_direction {
                directions::E if *c == 'J' => {
                    travel_direction = directions::N;
                }
                directions::E if *c == '7' => {
                    travel_direction = directions::S;
                }

                directions::W if *c == 'L' => {
                    travel_direction = directions::N;
                }
                directions::W if *c == 'F' => {
                    travel_direction = directions::S;
                }

                directions::N if *c == 'F' => {
                    travel_direction = directions::E;
                }
                directions::N if *c == '7' => {
                    travel_direction = directions::W;
                }

                directions::S if *c == 'L' => {
                    travel_direction = directions::E;
                }
                directions::S if *c == 'J' => {
                    travel_direction = directions::W;
                }

                _ => {}
            }
        }
    }

    points
}

#[allow(unused)]
fn print_area(input: &str, points: &Vec<(usize, usize)>) {
    let area = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            let mut inside = false;

            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    if points.contains(&(row, col)) {
                        if ['S', '|', '7', 'F'].contains(&c) {
                            inside = !inside;
                        }

                        c
                    } else {
                        if inside {
                            '#'
                        } else {
                            c
                        }
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>();

    println!("{}", area.join("\n"));
}

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    info!("processing input");

    let points = points(input);

    let filtered = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    if points.contains(&(y, x)) {
                        Some(c)
                    } else {
                        Some('.')
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // print_area(input, &points);

    let area = filtered.iter().enumerate().fold(0, |acc, (row, line)| {
        let mut inside = false;

        acc + line.iter().enumerate().fold(0, |inner_acc, (col, c)| {
            if points.contains(&(row, col)) {
                if ['S', '|', '7', 'F'].contains(&c) {
                    inside = !inside;
                }

                inner_acc
            } else {
                if inside {
                    info!(?row, ?col, ?c);
                    inner_acc + 1
                } else {
                    inner_acc
                }
            }
        })
    });

    let result = area;

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
    fn simple_loop() {
        let input = r##"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
"##;
        let result = process(input).unwrap();
        assert_eq!(result, "4");
    }

    #[test_log::test]
    fn complex_loop() {
        let input = r##".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
"##;
        let result = process(input).unwrap();
        assert_eq!(result, "8");
    }

    #[test_log::test]
    fn even_more_complex_loop() {
        let input = r##"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
"##;
        let result = process(input).unwrap();
        assert_eq!(result, "10");
    }
}
