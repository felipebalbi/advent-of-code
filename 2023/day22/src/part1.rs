use anyhow::{Context, Result};
use glam::IVec3;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, space0, space1},
    combinator::{map, recognize},
    multi::{many1, separated_list1},
    sequence::{pair, separated_pair, terminated, tuple},
    IResult,
};
use tracing::info;

type Coordinate = IVec3;

#[derive(Debug, Clone, Copy)]
struct Brick {
    start: Coordinate,
    end: Coordinate,
}

impl Brick {
    fn minimum_height(&self) -> i32 {
        self.start.z.min(self.end.z)
    }

    fn parallel(&self, other: &Brick) -> bool {
        let line1 = self.end - self.start;
        let line2 = other.start - other.end;

        line1.x * line2.y - line2.x * line1.y == 0
    }

    fn is_disintegratable(&self, bricks: &[Brick]) -> bool {
        let supporting = self.supporting(bricks);

        info!(?self, ?supporting);

        supporting.iter().all(|brick| {
            let count = brick.supporters(bricks).len();

            info!(?brick, ?count);

            count > 1
        })
    }

    fn supporting(&self, bricks: &[Brick]) -> Vec<Brick> {
        bricks
            .iter()
            .cloned()
            .filter(|brick| {
                let x_range = self.start.x..=self.end.x;
                let y_range = self.start.y..=self.end.y;

                // info!(?x_range, ?y_range, ?brick);

                (x_range.contains(&brick.start.x)
                    || x_range.contains(&brick.end.x)
                    || y_range.contains(&brick.start.y)
                    || y_range.contains(&brick.end.y))
                    && self.start.z.max(self.end.z) == brick.start.z.min(brick.end.z) - 1
            })
            .collect::<Vec<_>>()
    }

    fn supporters(&self, bricks: &[Brick]) -> Vec<Brick> {
        bricks
            .iter()
            .cloned()
            .filter(|brick| {
                let x_range = self.start.x..=self.end.x;
                let y_range = self.start.y..=self.end.y;

                // info!(?x_range, ?y_range, ?brick);

                (x_range.contains(&brick.start.x)
                    || x_range.contains(&brick.end.x)
                    || y_range.contains(&brick.start.y)
                    || y_range.contains(&brick.end.y))
                    && self.start.z.min(self.end.z) == brick.start.z.max(brick.end.z) + 1
            })
            .collect::<Vec<_>>()
    }
}

#[tracing::instrument(skip(input))]
fn coordinate(input: &str) -> IResult<&str, IVec3> {
    map(
        tuple((
            complete::i32,
            tag(","),
            complete::i32,
            tag(","),
            complete::i32,
        )),
        |(x, _, y, _, z)| IVec3::new(x, y, z),
    )(input)
}

#[tracing::instrument(skip(input))]
fn brick(input: &str) -> IResult<&str, Brick> {
    map(
        separated_pair(coordinate, tag("~"), coordinate),
        |(start, end)| Brick { start, end },
    )(input)
}

#[tracing::instrument(skip(input))]
fn bricks(input: &str) -> IResult<&str, Vec<Brick>> {
    separated_list1(line_ending, brick)(input)
}

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    info!("processing input");

    let (_, mut bricks) = bricks(input)?;

    bricks.sort_by(|a, b| {
        a.start
            .z
            .min(a.end.z)
            .partial_cmp(&b.start.z.min(b.end.z))
            .expect("should be comparable")
    });

    let _ = bricks
        .iter()
        .tuple_windows()
        .map(|(a, b)| {
            let parallel = a.parallel(b);

            info!(?a, ?b, ?parallel);
        })
        .collect::<Vec<_>>();

    // let settled = bricks
    //     .into_iter()
    //     .map(|mut brick| {
    //         let min_z = brick.start.z.min(brick.end.z);

    //         if min_z > minimum_height {
    //             let diff = min_z - minimum_height;

    //             brick.start.z -= diff;
    //             brick.end.z -= diff;
    //         }

    //         minimum_height += 1;

    //         brick
    //     })
    //     .collect::<Vec<_>>();

    // let result = settled
    //     .iter()
    //     .filter(|brick| brick.is_disintegratable(&settled))
    //     .count();

    let result = 0;
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
        let input = r##"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"##;
        let result = process(input).unwrap();
        assert_eq!(result, "5");
    }
}
