use anyhow::{Context, Result};
use itertools::{repeat_n, Itertools};
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, one_of, space1},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};
use tracing::info;

#[derive(Debug, PartialEq, Clone)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug)]
struct Row {
    conditions: Vec<Condition>,
    groups: Vec<u32>,
}

impl Row {
    fn unfold(&self) -> Row {
        let conditions = repeat_n(self.conditions.clone(), 5)
            .flatten()
            .collect::<Vec<_>>();

        let groups = repeat_n(self.groups.clone(), 5)
            .flatten()
            .collect::<Vec<_>>();

        Row { conditions, groups }
    }

    fn permute(&self) -> Vec<Vec<Condition>> {
        let unknowns = self
            .conditions
            .iter()
            .filter(|condition| *condition == &Condition::Unknown)
            .count();
        repeat_n([Condition::Operational, Condition::Damaged], unknowns)
            .multi_cartesian_product()
            .collect::<Vec<_>>()
    }

    fn is_valid(&self, permutation: &Vec<Condition>) -> bool {
        let mut it = permutation.iter();
        let groups = self
            .conditions
            .iter()
            .map(|condition| match condition {
                Condition::Unknown => it.next().expect("should have a valid permutation"),
                c => c,
            })
            .group_by(|condition| *condition == &Condition::Damaged)
            .into_iter()
            .filter_map(|(is_damaged, group)| {
                is_damaged.then_some(group.into_iter().count() as u32)
            })
            .collect::<Vec<u32>>();

        &self.groups[..] == &groups[..]
    }
}

#[tracing::instrument(skip(input))]
fn conditions(input: &str) -> IResult<&str, Vec<Condition>> {
    many1(map(one_of("?.#"), |c| match c {
        '?' => Condition::Unknown,
        '.' => Condition::Operational,
        '#' => Condition::Damaged,
        _ => unreachable!(),
    }))(input)
}

#[tracing::instrument(skip(input))]
fn groups(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(","), complete::u32)(input)
}

#[tracing::instrument(skip(input))]
fn row(input: &str) -> IResult<&str, Row> {
    map(
        separated_pair(conditions, space1, groups),
        |(conditions, groups)| Row { conditions, groups },
    )(input)
}

#[tracing::instrument(skip(input))]
fn records(input: &str) -> IResult<&str, Vec<Row>> {
    separated_list1(line_ending, row)(input)
}

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    info!("processing input");

    // let (_, records) = records(input)?;

    // info!(?records);

    // let result = records
    //     .iter()
    //     .map(|row| {
    //         row.unfold()
    //             .permute()
    //             .iter()
    //             .filter(|permutation| row.is_valid(permutation))
    //             .count()
    //     })
    //     .sum::<usize>();

    let result = 0;
    Ok(result.to_string())
}

#[tracing::instrument(skip(input))]
pub fn part2(input: &'static str) -> Result<String> {
    info!("part s");

    process(input).context("process part 2")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn line1() {
        let input = "???.### 1,1,3\n";
        let result = process(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "1");
    }

    #[test_log::test]
    fn line2() {
        let input = ".??..??...?##. 1,1,3\n";
        let result = process(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "16384");
    }

    #[test_log::test]
    fn line3() {
        let input = "?#?#?#?#?#?#?#? 1,3,1,6\n";
        let result = process(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "1");
    }

    #[test_log::test]
    fn line4() {
        let input = "????.#...#... 4,1,1\n";
        let result = process(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "16");
    }
    #[test_log::test]
    fn line5() {
        let input = "????.######..#####. 1,6,5\n";
        let result = process(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "2500");
    }
    #[test_log::test]
    fn line6() {
        let input = "?###???????? 3,2,1\n";
        let result = process(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "506250");
    }

    #[test_log::test]
    fn all_lines() {
        let input = r##"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"##;
        let result = process(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "525152");
    }
}
