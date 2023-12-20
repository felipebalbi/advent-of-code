use anyhow::{Context, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use std::collections::{BTreeMap, VecDeque};
use tracing::info;

#[derive(Debug, Clone)]
enum Module<'a> {
    Broadcaster {
        counts: (usize, usize),
        outputs: Vec<&'a str>,
    },
    Conjunction {
        counts: (usize, usize),
        state: bool,
        outputs: Vec<&'a str>,
        inputs: BTreeMap<&'a str, bool>,
    },
    FlipFlop {
        counts: (usize, usize),
        state: bool,
        outputs: Vec<&'a str>,
    },
}

#[tracing::instrument(skip(input))]
fn outputs(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(", "), alpha1)(input)
}

#[tracing::instrument(skip(input))]
fn broadcaster(input: &str) -> IResult<&str, (&str, Module)> {
    map(
        tuple((tag("broadcaster"), tag(" -> "), outputs)),
        |(name, _, outputs)| {
            (
                name,
                Module::Broadcaster {
                    counts: (0, 0),
                    outputs,
                },
            )
        },
    )(input)
}

#[tracing::instrument(skip(input))]
fn conjunction(input: &str) -> IResult<&str, (&str, Module)> {
    map(
        tuple((tag("&"), alpha1, tag(" -> "), outputs)),
        |(_, name, _, outputs)| {
            (
                name,
                Module::Conjunction {
                    counts: (0, 0),
                    state: false,
                    inputs: BTreeMap::new(),
                    outputs,
                },
            )
        },
    )(input)
}

#[tracing::instrument(skip(input))]
fn flipflop(input: &str) -> IResult<&str, (&str, Module)> {
    map(
        tuple((tag("%"), alpha1, tag(" -> "), outputs)),
        |(_, name, _, outputs)| {
            (
                name,
                Module::FlipFlop {
                    counts: (0, 0),
                    state: false,
                    outputs,
                },
            )
        },
    )(input)
}

#[tracing::instrument(skip(input))]
fn module(input: &str) -> IResult<&str, (&str, Module)> {
    alt((broadcaster, conjunction, flipflop))(input)
}

#[tracing::instrument(skip(input))]
fn modules(input: &str) -> IResult<&str, BTreeMap<&str, Module>> {
    map(separated_list1(line_ending, module), |modules| {
        BTreeMap::from_iter(modules)
    })(input)
}

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    info!("processing input");

    let (_, mut map) = modules(input)?;

    let mut conjunctions = map
        .iter()
        .filter(|(name, module)| match module {
            Module::Conjunction { .. } => true,
            _ => false,
        })
        .collect::<Vec<_>>();

    for (cname, conjunction) in conjunctions.iter_mut() {
        for (name, module) in map.iter() {
            match module {
                Module::Broadcaster { outputs, .. }
                | Module::Conjunction { outputs, .. }
                | Module::FlipFlop { outputs, .. } => match conjunction {
                    Module::Conjunction { inputs, .. } => {
                        if outputs.contains(cname) {
                            inputs.insert(name, false);
                        }
                    }
                    _ => unreachable!(),
                },
            }
        }
    }
    let mut queue = VecDeque::new();

    queue.push_back("broadcaster");

    let mut counts = (0, 0); // low, high

    while !queue.is_empty() {
        let name = queue.pop_front().expect("should have a node name");
        let module = map.get_mut(name).expect("should have a node");

        match module {
            Module::Broadcaster { outputs, .. } => {
                counts.0 += 1;
                queue.extend(outputs.iter());
            }
            Module::FlipFlop { .. } => {
                // if !signal {
                //     counts.0 += 1;
                //     queue.extend(outputs.iter());
                // } else {
                //     counts.1 += 1;
                // }
            }
            Module::Conjunction {
                counts, outputs, ..
            } => {
                queue.extend(outputs.iter());
            }
        }
    }

    Ok("".to_string())
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
    fn example1() {
        let input = r##"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"##;
        let result = process(input).unwrap();
        assert_eq!(result, "32000000");
    }

    #[test_log::test]
    fn example2() {
        let input = r##"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"##;
        let result = process(input).unwrap();
        assert_eq!(result, "11687500");
    }
}
