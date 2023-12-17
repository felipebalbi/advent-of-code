use anyhow::{Context, Result};
use glam::IVec2;
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
    position: IVec2,
    next_position: VecDeque<(IVec2, Direction)>,
    path: HashSet<(IVec2, Direction)>,
    energized: HashSet<IVec2>,
}

impl Ray {
    fn new() -> Self {
        let position = IVec2::new(0, 0);
        let direction = Direction::Right;
        let next_position = (position, direction);

        let mut ray = Ray {
            direction,
            position,
            next_position: VecDeque::new(),
            path: HashSet::new(),
            energized: HashSet::new(),
        };

        ray.next_position.push_back(next_position);

        ray
    }

    fn calculate_next_positions(&mut self, grid: &[u8], width: usize, height: usize) {
        let (position, direction) = self
            .next_position
            .pop_back()
            .expect("should have next position");

        self.position = position;
        self.direction = direction;

        let tile = grid[self.grid_position(width)];

        // energize current tile
        self.energized.insert(self.position);

        // add current tile to my path
        self.path.insert((self.position, self.direction));

        // add next position to the queue
        match tile {
            // Empty
            b'.' => self.follow_empty(width as isize, height as isize),

            // Mirrors
            b'/' | b'\\' => self.follow_mirror(tile, width as isize, height as isize),

            // Splitters
            b'-' | b'|' => self.follow_splitter(tile, width as isize, height as isize),

            // ignore anything else
            _ => {}
        }
    }

    fn append_next_position(
        &mut self,
        position: IVec2,
        direction: Direction,
        width: isize,
        height: isize,
    ) {
        if self.is_within_bounds(position, width, height)
            && !self.path.contains(&(position, direction))
        {
            self.next_position.push_back((position, direction));
        }
    }

    fn follow_empty(&mut self, width: isize, height: isize) {
        match self.direction {
            Direction::Up => self.append_next_position(
                self.position + IVec2::new(0, -1),
                Direction::Up,
                width,
                height,
            ),
            Direction::Down => self.append_next_position(
                self.position + IVec2::new(0, 1),
                Direction::Down,
                width,
                height,
            ),
            Direction::Left => self.append_next_position(
                self.position + IVec2::new(-1, 0),
                Direction::Left,
                width,
                height,
            ),
            Direction::Right => self.append_next_position(
                self.position + IVec2::new(1, 0),
                Direction::Right,
                width,
                height,
            ),
        }
    }

    fn follow_splitter(&mut self, tile: u8, width: isize, height: isize) {
        match tile {
            b'|' => match self.direction {
                Direction::Left | Direction::Right => {
                    self.append_next_position(
                        self.position + IVec2::new(0, -1),
                        Direction::Up,
                        width,
                        height,
                    );
                    self.append_next_position(
                        self.position + IVec2::new(0, 1),
                        Direction::Down,
                        width,
                        height,
                    );
                }
                _ => self.follow_empty(width, height),
            },
            b'-' => match self.direction {
                Direction::Up | Direction::Down => {
                    self.append_next_position(
                        self.position + IVec2::new(-1, 0),
                        Direction::Left,
                        width,
                        height,
                    );
                    self.append_next_position(
                        self.position + IVec2::new(1, 0),
                        Direction::Right,
                        width,
                        height,
                    );
                }
                _ => self.follow_empty(width, height),
            },
            _ => unreachable!(),
        }
    }

    fn follow_mirror(&mut self, tile: u8, width: isize, height: isize) {
        match tile {
            b'/' => match self.direction {
                Direction::Up => self.append_next_position(
                    self.position + IVec2::new(1, 0),
                    Direction::Right,
                    width,
                    height,
                ),
                Direction::Down => self.append_next_position(
                    self.position + IVec2::new(-1, 0),
                    Direction::Left,
                    width,
                    height,
                ),
                Direction::Left => self.append_next_position(
                    self.position + IVec2::new(0, 1),
                    Direction::Down,
                    width,
                    height,
                ),
                Direction::Right => self.append_next_position(
                    self.position + IVec2::new(0, -1),
                    Direction::Up,
                    width,
                    height,
                ),
            },
            b'\\' => match self.direction {
                Direction::Up => self.append_next_position(
                    self.position + IVec2::new(-1, 0),
                    Direction::Left,
                    width,
                    height,
                ),
                Direction::Down => self.append_next_position(
                    self.position + IVec2::new(1, 0),
                    Direction::Right,
                    width,
                    height,
                ),
                Direction::Left => self.append_next_position(
                    self.position + IVec2::new(0, -1),
                    Direction::Up,
                    width,
                    height,
                ),
                Direction::Right => self.append_next_position(
                    self.position + IVec2::new(0, 1),
                    Direction::Down,
                    width,
                    height,
                ),
            },
            _ => unreachable!(),
        }
    }

    fn keep_following(&self) -> bool {
        !self.next_position.is_empty()
    }

    fn grid_position(&self, width: usize) -> usize {
        (self.position.y as usize) * width + self.position.x as usize
    }

    fn is_within_bounds(&self, position: IVec2, width: isize, height: isize) -> bool {
        position.x >= 0
            && position.x < (width as i32) - 1 // ignore \n at the end of each line
            && position.y >= 0
            && position.y < (height as i32)
    }
}

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    info!("processing input");

    let width = input.find('\n').expect("should have a newline") + 1;
    let height = input.len() / width;
    let grid = input.as_bytes();
    let mut ray = Ray::new();

    while ray.keep_following() {
        ray.calculate_next_positions(grid, width, height);
    }

    let energized_count = ray.energized.len();
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
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";
        let result = process(input).unwrap();
        assert_eq!(result, "46");
    }
}
