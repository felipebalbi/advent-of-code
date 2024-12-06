use anyhow::{Context, Result};
use rayon::prelude::*;
use tracing::info;

#[tracing::instrument(skip(v))]
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());

    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

#[tracing::instrument(skip(pattern))]
fn find_reflection_line(pattern: Vec<Vec<char>>) -> usize {
    let length = pattern.len();

    (1..length).fold(0, |mut acc, line_number| {
        let (left, right) = pattern.split_at(line_number);
        let mut left = left.iter().clone().collect::<Vec<_>>();
        let right = right.iter().collect::<Vec<_>>();

        left.reverse();

        if right
            .iter()
            .zip(&left)
            .map(|(a, b)| a.iter().zip(b.iter()).filter(|(a, b)| a != b).count())
            .sum::<usize>()
            == 1
        {
            acc += line_number;
        }

        acc
    })
}

#[tracing::instrument(skip(pattern))]
fn reflection(pattern: &str) -> usize {
    let lines = pattern
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let vertical = find_reflection_line(transpose(lines.clone()));
    let horizontal = find_reflection_line(lines) * 100;
    horizontal + vertical
}

#[tracing::instrument(skip(input))]
fn patterns(input: &str) -> Vec<&str> {
    input.split("\n\n").collect::<Vec<_>>()
}

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    info!("processing input");

    let patterns = patterns(input);

    let result = patterns
        .par_iter()
        .map(|pattern| reflection(pattern))
        .sum::<usize>();

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
        let input = r##"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"##;
        let result = process(input).unwrap();
        assert_eq!(result, "400");
    }
}
