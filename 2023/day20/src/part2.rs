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
use std::{
    cmp::Ordering,
    collections::{BTreeMap, VecDeque},
};
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

fn gcd(a: usize, b: usize) -> usize {
    match (a, b) {
        (a, 0) => a,
        (0, b) => b,
        (a, b) => match a.cmp(&b) {
            Ordering::Less => gcd(a, b % a),
            _ => gcd(b, a % b),
        },
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    info!("processing input");

    let (_, mut map) = modules(input)?;

    // We want rx to receive a low
    let final_node = "rx";

    // Therefore, we find the node that sends a signal to rx
    let rx_source = map
        .iter()
        .find_map(|(id, module)| module.outputs.contains(&final_node).then_some(*id))
        .unwrap();

    // And find its source
    let mut rx_source_sources = map
        .iter()
        .filter_map(|(id, module)| module.outputs.contains(&rx_source).then_some(*id))
        .collect::<Vec<&str>>();

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
    let mut minimum_pushes = Vec::new();

    for i in 0.. {
        queue.push_back(("button".to_string(), "broadcaster".to_string(), Signal::Low));

        if minimum_pushes.len() == 4 {
            break;
        }

        while let Some((src, dst, signal)) = queue.pop_front() {
            if rx_source_sources.contains(&dst.as_str()) && signal == Signal::Low {
                let index = rx_source_sources.iter().position(|x| x == &dst).unwrap();

                rx_source_sources.remove(index);
                minimum_pushes.push(i + 1);
            }

            let output = map
                .get_mut(dst.as_str())
                .map(|module| module.process_signal(src.clone(), &signal))
                .unwrap_or(vec![]);

            queue.extend(output);
        }
    }

    let minimum = minimum_pushes
        .into_iter()
        .reduce(lcm)
        .expect("should be a number");

    Ok(minimum.to_string())
}

#[tracing::instrument(skip(input))]
pub fn part2(input: &'static str) -> Result<String> {
    info!("part 2");

    process(input).context("process part 2")
}
