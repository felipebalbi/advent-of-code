use anyhow::{Context, Result};
use petgraph::{algo::dijkstra, prelude::*};
use tracing::info;

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
enum Tile {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl Default for Tile {
    fn default() -> Self {
        Self::Ground
    }
}

#[tracing::instrument(skip(input))]
fn maze(input: &str) -> Vec<Tile> {
    input
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '|' => Tile::NorthSouth,
                '-' => Tile::EastWest,
                'L' => Tile::NorthEast,
                'J' => Tile::NorthWest,
                '7' => Tile::SouthWest,
                'F' => Tile::SouthEast,
                '.' => Tile::Ground,
                'S' => Tile::Start,
                _ => unreachable!(),
            })
        })
        .collect::<Vec<_>>()
}

#[tracing::instrument(skip(maze))]
fn build_graph(maze: &[Tile]) -> (Graph<(), (), Directed>, (Tile, NodeIndex)) {
    let mut graph: Graph<(), (), Directed> = Graph::new();
    let nodes = maze
        .iter()
        .map(|tile| {
            let node = graph.add_node(());
            (*tile, node)
        })
        .collect::<Vec<_>>();

    let starting_node = nodes
        .iter()
        .find(|(tile, _)| tile == &Tile::Start)
        .expect("must have a starting node");

    let length = (maze.len() as f32).sqrt() as usize;

    for (i, (tile, node)) in nodes.iter().enumerate() {
        match tile {
            Tile::NorthSouth => {
                nodes
                    .get(i + length)
                    .and_then(|neighbor| Some(graph.add_edge(*node, neighbor.1, ())));
                if i >= length {
                    nodes
                        .get(i - length)
                        .and_then(|neighbor| Some(graph.add_edge(*node, neighbor.1, ())));
                }
            }
            Tile::EastWest => {
                nodes
                    .get(i + 1)
                    .and_then(|neighbor| Some(graph.add_edge(*node, neighbor.1, ())));
                if i >= 1 {
                    nodes
                        .get(i - 1)
                        .and_then(|neighbor| Some(graph.add_edge(*node, neighbor.1, ())));
                }
            }

            Tile::NorthEast => {
                if i >= length {
                    nodes
                        .get(i - length)
                        .and_then(|neighbor| Some(graph.add_edge(*node, neighbor.1, ())));
                }
                nodes
                    .get(i + 1)
                    .and_then(|neighbor| Some(graph.add_edge(*node, neighbor.1, ())));
            }

            Tile::NorthWest => {
                if i >= length {
                    nodes
                        .get(i - length)
                        .and_then(|neighbor| Some(graph.add_edge(*node, neighbor.1, ())));
                }
                if i >= 1 {
                    nodes
                        .get(i - 1)
                        .and_then(|neighbor| Some(graph.add_edge(*node, neighbor.1, ())));
                }
            }

            Tile::SouthWest => {
                nodes
                    .get(i + length)
                    .and_then(|neighbor| Some(graph.add_edge(*node, neighbor.1, ())));
                if i >= 1 {
                    nodes
                        .get(i - 1)
                        .and_then(|neighbor| Some(graph.add_edge(*node, neighbor.1, ())));
                }
            }

            Tile::SouthEast => {
                nodes
                    .get(i + 1)
                    .and_then(|neighbor| Some(graph.add_edge(*node, neighbor.1, ())));
                nodes
                    .get(i + length)
                    .and_then(|neighbor| Some(graph.add_edge(*node, neighbor.1, ())));
            }

            Tile::Start => {
                nodes.get(i + 1).and_then(|neighbor| {
                    if neighbor.0 == Tile::EastWest
                        || neighbor.0 == Tile::SouthWest
                        || neighbor.0 == Tile::NorthWest
                    {
                        Some(graph.add_edge(*node, neighbor.1, ()))
                    } else {
                        None
                    }
                });

                if i >= 1 {
                    nodes.get(i - 1).and_then(|neighbor| {
                        if neighbor.0 == Tile::EastWest
                            || neighbor.0 == Tile::SouthEast
                            || neighbor.0 == Tile::NorthEast
                        {
                            Some(graph.add_edge(*node, neighbor.1, ()))
                        } else {
                            None
                        }
                    });
                }
                nodes.get(i + length).and_then(|neighbor| {
                    if neighbor.0 == Tile::NorthSouth
                        || neighbor.0 == Tile::NorthWest
                        || neighbor.0 == Tile::NorthEast
                    {
                        Some(graph.add_edge(*node, neighbor.1, ()))
                    } else {
                        None
                    }
                });
                if i >= length {
                    nodes.get(i - length).and_then(|neighbor| {
                        if neighbor.0 == Tile::NorthSouth
                            || neighbor.0 == Tile::SouthWest
                            || neighbor.0 == Tile::SouthEast
                        {
                            Some(graph.add_edge(*node, neighbor.1, ()))
                        } else {
                            None
                        }
                    });
                }
            }
            _ => {}
        }
    }

    (graph, *starting_node)
}

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    info!("processing input");

    let maze = maze(input);
    let (graph, start) = build_graph(&maze);
    let res = dijkstra(&graph, start.1, None, |_| 1);
    let max = res.values().max().unwrap();

    Ok(max.to_string())
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
    fn simple_loop() {
        let input = r##".....
.S-7.
.|.|.
.L-J.
.....
"##;
        let result = process(input).unwrap();
        assert_eq!(result, "4");
    }

    #[test_log::test]
    fn complex_loop() {
        let input = r##"..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"##;
        let result = process(input).unwrap();
        assert_eq!(result, "8");
    }
}
