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
enum Status {
    On,
    Off,
}

#[derive(Debug, Clone, PartialEq, Copy)]
enum Signal {
    High,
    Low,
}

#[derive(Debug, Clone)]
enum ModuleType<'a> {
    Broadcaster,
    Conjunction { inputs: BTreeMap<&'a str, Signal> },
    FlipFlop { status: Status },
}

#[derive(Debug)]
struct Module<'a> {
    name: &'a str,
    outputs: Vec<&'a str>,
    module_type: ModuleType<'a>,
}

impl<'a> Module<'a> {
    fn process_signal(&mut self, src: String, signal: &Signal) -> Vec<(String, String, Signal)> {
        match &mut self.module_type {
            ModuleType::Broadcaster => self
                .outputs
                .iter()
                .map(|dst| (self.name.to_string(), dst.to_string(), *signal))
                .collect::<Vec<(String, String, Signal)>>(),
            ModuleType::Conjunction { ref mut inputs } => {
                // if it remembers high pulses for all inputs, it sends
                // a low pulse; otherwise, it sends a high pulse.
                *inputs.get_mut(src.as_str()).unwrap() = *signal;

                let output_signal = inputs
                    .values()
                    .all(|signal| signal == &Signal::High)
                    .then_some(Signal::Low)
                    .unwrap_or(Signal::High);

                self.outputs
                    .iter()
                    .map(|dst| (self.name.to_string(), dst.to_string(), output_signal))
                    .collect::<Vec<(String, String, Signal)>>()
            }
            ModuleType::FlipFlop { ref mut status } => match (&status, signal) {
                (_, &Signal::High) => vec![],
                (&Status::On, &Signal::Low) => {
                    *status = Status::Off;

                    self.outputs
                        .iter()
                        .map(|dst| (self.name.to_string(), dst.to_string(), Signal::Low))
                        .collect::<Vec<(String, String, Signal)>>()
                }
                (&Status::Off, &Signal::Low) => {
                    *status = Status::On;

                    self.outputs
                        .iter()
                        .map(|dst| (self.name.to_string(), dst.to_string(), Signal::High))
                        .collect::<Vec<(String, String, Signal)>>()
                }
            },
        }
    }
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
                Module {
                    name,
                    outputs,
                    module_type: ModuleType::Broadcaster,
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
                Module {
                    name,
                    outputs,
                    module_type: ModuleType::Conjunction {
                        inputs: BTreeMap::new(),
                    },
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
                Module {
                    name,
                    outputs,
                    module_type: ModuleType::FlipFlop {
                        status: Status::Off,
                    },
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

    let conjunctions = map
        .iter()
        .filter_map(|(name, module)| match module.module_type {
            ModuleType::Conjunction { .. } => Some(name),
            _ => None,
        })
        .collect::<Vec<_>>();

    let inputs = map.iter().fold(
        BTreeMap::<&str, Vec<&str>>::new(),
        |mut acc, (id, module)| {
            for conjunction in conjunctions.iter() {
                if module.outputs.contains(conjunction) {
                    acc.entry(conjunction)
                        .and_modify(|item| {
                            item.push(id);
                        })
                        .or_insert(vec![id]);
                }
            }
            acc
        },
    );

    inputs.into_iter().for_each(|(conjunction, input_modules)| {
        map.entry(conjunction).and_modify(|module| {
            if let ModuleType::Conjunction { inputs, .. } = &mut module.module_type {
                *inputs = input_modules
                    .into_iter()
                    .map(|id| (id, Signal::Low))
                    .collect();
            } else {
                unreachable!("has to exist");
            };
        });
    });

    let mut queue = VecDeque::new();

    let button_presses = 1000;
    let mut high_count = 0;
    let mut low_count = 0;

    for _ in 0..button_presses {
        queue.push_back(("button".to_string(), "broadcaster".to_string(), Signal::Low));
        low_count += 1;

        while let Some((src, dst, signal)) = queue.pop_front() {
            info!(?high_count, ?low_count);

            let output = map
                .get_mut(dst.as_str())
                .map(|module| module.process_signal(src.clone(), &signal))
                .unwrap_or(vec![]);

            for (_, _, signal) in output.iter() {
                match signal {
                    Signal::High => {
                        high_count += 1;
                    }
                    Signal::Low => {
                        low_count += 1;
                    }
                }
            }

            info!(?output, "{dst} ->");

            queue.extend(output);
        }
    }

    Ok((high_count * low_count).to_string())
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
