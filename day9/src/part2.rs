use anyhow::{Context, Result};
use nom::{
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    IResult,
};
use tracing::info;

#[tracing::instrument(skip(input))]
fn history(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(space1, complete::i64)(input)
}

#[tracing::instrument(skip(input))]
fn oasis(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    separated_list1(line_ending, history)(input)
}

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    info!("processing input");

    let (_, oasis) = oasis(input)?;

    info!(?oasis);

    let result = oasis
        .iter()
        .map(|history| {
            let mut grid = vec![history.clone()];

            while !grid.last().unwrap().iter().all(|n| *n == 0) {
                let step = grid
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|w| {
                        info!(?w);

                        w[1] - w[0]
                    })
                    .collect::<Vec<_>>();
                grid.push(step);
            }

            grid.iter()
                .map(|ns| ns.first().unwrap())
                .rev()
                .fold(0, |mut acc, n| {
                    acc = n - acc;
                    acc
                })
        })
        .sum::<i64>();

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
        let input = r##"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"##;
        let result = process(input).unwrap();
        assert_eq!(result, "2");
    }
}
