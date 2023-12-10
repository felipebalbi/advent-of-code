use anyhow::{Context, Result};
use petgraph::{algo::dijkstra, prelude::*, visit::Dfs};
use tracing::info;

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
enum Pipe {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
struct Tile {
    x: usize,
    y: usize,
    pipe: Pipe,
}

impl Tile {
    fn new(x: usize, y: usize, pipe: Pipe) -> Self {
        Self { x, y, pipe }
    }

    fn area(&self, other: &Tile) -> f32 {
        // 0.5 * (self.y as f32 + other.y as f32) * (self.x as f32 - other.x as f32)
        0.5 * ((self.x * other.y) as f32 - (other.x * self.y) as f32)
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            x: 0,
            y: 0,
            pipe: Pipe::Ground,
        }
    }
}

#[tracing::instrument(skip(input))]
fn maze(input: &str) -> Vec<Tile> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| match c {
                '|' => Tile::new(x + 1, y + 1, Pipe::NorthSouth),
                '-' => Tile::new(x + 1, y + 1, Pipe::EastWest),
                'L' => Tile::new(x + 1, y + 1, Pipe::NorthEast),
                'J' => Tile::new(x + 1, y + 1, Pipe::NorthWest),
                '7' => Tile::new(x + 1, y + 1, Pipe::SouthWest),
                'F' => Tile::new(x + 1, y + 1, Pipe::SouthEast),
                '.' => Tile::new(x + 1, y + 1, Pipe::Ground),
                'S' => Tile::new(x + 1, y + 1, Pipe::Start),
                _ => unreachable!(),
            })
        })
        .collect::<Vec<_>>()
}

#[tracing::instrument(skip(maze))]
fn build_graph(maze: &[Tile]) -> (Graph<(), f32, Directed>, Vec<(Tile, NodeIndex)>) {
    let mut graph: Graph<(), f32, Directed> = Graph::new();
    let nodes = maze
        .iter()
        .map(|tile| {
            let node = graph.add_node(());
            (*tile, node)
        })
        .collect::<Vec<_>>();

    let length = (maze.len() as f32).sqrt() as usize;

    for (i, (tile, node)) in nodes.iter().enumerate() {
        match tile.pipe {
            Pipe::NorthSouth => {
                nodes.get(i + length).and_then(|neighbor| {
                    Some(graph.add_edge(*node, neighbor.1, tile.area(&neighbor.0)))
                });
                if i >= length {
                    nodes.get(i - length).and_then(|neighbor| {
                        Some(graph.add_edge(*node, neighbor.1, tile.area(&neighbor.0)))
                    });
                }
            }
            Pipe::EastWest => {
                nodes.get(i + 1).and_then(|neighbor| {
                    Some(graph.add_edge(*node, neighbor.1, tile.area(&neighbor.0)))
                });
                if i >= 1 {
                    nodes.get(i - 1).and_then(|neighbor| {
                        Some(graph.add_edge(*node, neighbor.1, tile.area(&neighbor.0)))
                    });
                }
            }

            Pipe::NorthEast => {
                if i >= length {
                    nodes.get(i - length).and_then(|neighbor| {
                        Some(graph.add_edge(*node, neighbor.1, tile.area(&neighbor.0)))
                    });
                }
                nodes.get(i + 1).and_then(|neighbor| {
                    Some(graph.add_edge(*node, neighbor.1, tile.area(&neighbor.0)))
                });
            }

            Pipe::NorthWest => {
                if i >= length {
                    nodes.get(i - length).and_then(|neighbor| {
                        Some(graph.add_edge(*node, neighbor.1, tile.area(&neighbor.0)))
                    });
                }
                if i >= 1 {
                    nodes.get(i - 1).and_then(|neighbor| {
                        Some(graph.add_edge(*node, neighbor.1, tile.area(&neighbor.0)))
                    });
                }
            }

            Pipe::SouthWest => {
                nodes.get(i + length).and_then(|neighbor| {
                    Some(graph.add_edge(*node, neighbor.1, tile.area(&neighbor.0)))
                });
                if i >= 1 {
                    nodes.get(i - 1).and_then(|neighbor| {
                        Some(graph.add_edge(*node, neighbor.1, tile.area(&neighbor.0)))
                    });
                }
            }

            Pipe::SouthEast => {
                nodes.get(i + 1).and_then(|neighbor| {
                    Some(graph.add_edge(*node, neighbor.1, tile.area(&neighbor.0)))
                });
                nodes.get(i + length).and_then(|neighbor| {
                    Some(graph.add_edge(*node, neighbor.1, tile.area(&neighbor.0)))
                });
            }

            Pipe::Start => {
                nodes.get(i + 1).and_then(|neighbor| {
                    if neighbor.0.pipe == Pipe::EastWest
                        || neighbor.0.pipe == Pipe::SouthWest
                        || neighbor.0.pipe == Pipe::NorthWest
                    {
                        Some(graph.add_edge(*node, neighbor.1, tile.area(&neighbor.0)))
                    } else {
                        None
                    }
                });

                if i >= 1 {
                    nodes.get(i - 1).and_then(|neighbor| {
                        if neighbor.0.pipe == Pipe::EastWest
                            || neighbor.0.pipe == Pipe::SouthEast
                            || neighbor.0.pipe == Pipe::NorthEast
                        {
                            Some(graph.add_edge(*node, neighbor.1, tile.area(&neighbor.0)))
                        } else {
                            None
                        }
                    });
                }
                nodes.get(i + length).and_then(|neighbor| {
                    if neighbor.0.pipe == Pipe::NorthSouth
                        || neighbor.0.pipe == Pipe::NorthWest
                        || neighbor.0.pipe == Pipe::NorthEast
                    {
                        Some(graph.add_edge(*node, neighbor.1, tile.area(&neighbor.0)))
                    } else {
                        None
                    }
                });
                if i >= length {
                    nodes.get(i - length).and_then(|neighbor| {
                        if neighbor.0.pipe == Pipe::NorthSouth
                            || neighbor.0.pipe == Pipe::SouthWest
                            || neighbor.0.pipe == Pipe::SouthEast
                        {
                            Some(graph.add_edge(*node, neighbor.1, tile.area(&neighbor.0)))
                        } else {
                            None
                        }
                    });
                }
            }
            _ => {}
        }
    }

    (graph, nodes)
}

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    info!("processing input");

    let maze = maze(input);
    let (graph, nodes) = build_graph(&maze);
    let start = nodes
        .iter()
        .find(|(tile, _)| tile.pipe == Pipe::Start)
        .expect("must have a starting node");

    info!(?graph);

    let mut area = 0.0;

    let mut dfs = Dfs::new(&graph, start.1);
    let mut last_index = start.1;

    while let Some(source) = dfs.next(&graph) {
        let mut edges = graph.neighbors_directed(source, Outgoing).detach();

        last_index = source;

        while let Some((edge, target)) = edges.next(&graph) {
            let weight = graph.edge_weight(edge).expect("edge must have weight");

            info!(?source, ?target, ?weight);

            area += weight;
        }
    }

    let end = nodes
        .iter()
        .find(|(_, index)| *index == last_index)
        .expect("must have an end node");

    info!(?start, ?end);

    area += end.0.area(&start.0);

    info!(?area);

    Ok(area.to_string())
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

    #[ignore]
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

    #[ignore]
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
