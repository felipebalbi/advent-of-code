use anyhow::{Context, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, one_of},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};
use std::{cmp::Ordering, collections::BTreeMap, ops::RangeInclusive};
use tracing::info;

#[derive(Debug)]
enum Target<'a> {
    Accept,
    Reject,
    Workflow(&'a str),
}

#[derive(Debug)]
enum ApplyResult<'a> {
    SplitRange {
        pass: (Part, &'a Target<'a>),
        fail: Part,
    },
    Pass(&'a Target<'a>),
    Fail,
}

#[derive(Debug)]
enum Rule<'a> {
    Test {
        category: char,
        ordering: Ordering,
        number: usize,
        target: Target<'a>,
    },
    Target(Target<'a>),
}

impl<'a> Rule<'a> {
    fn apply(&self, part: &Part) -> ApplyResult {
        match self {
            Rule::Target(target) => ApplyResult::Pass(target),
            Rule::Test {
                category,
                ordering,
                number,
                target,
            } => {
                let part_range = match *category {
                    'x' => &part.x,
                    'm' => &part.m,
                    'a' => &part.a,
                    's' => &part.s,
                    _ => unreachable!(),
                };

                if part_range.contains(number) {
                    // If the range contains the number, then we need
                    // to produce two ranges:
                    //
                    // 1. start()..number
                    // 2. number..end()
                    //
                    // The resulting ranges are a function of the
                    // ordering, so let's match

                    match ordering {
                        &Ordering::Less => {
                            let range_low = *part_range.start()..=(*number - 1);
                            let range_high = *number..=*part_range.end();

                            let mut part_low = part.clone();
                            let mut part_high = part.clone();

                            match *category {
                                'x' => {
                                    part_low.x = range_low;
                                    part_high.x = range_high;
                                }
                                'm' => {
                                    part_low.m = range_low;
                                    part_high.m = range_high;
                                }
                                'a' => {
                                    part_low.a = range_low;
                                    part_high.a = range_high;
                                }
                                's' => {
                                    part_low.s = range_low;
                                    part_high.s = range_high;
                                }
                                _ => unreachable!(),
                            };

                            ApplyResult::SplitRange {
                                pass: (part_low, target),
                                fail: part_high,
                            }
                        }
                        &Ordering::Greater => {
                            let range_low = *part_range.start()..=*number;
                            let range_high = (*number + 1)..=*part_range.end();

                            let mut part_low = part.clone();
                            let mut part_high = part.clone();

                            match *category {
                                'x' => {
                                    part_low.x = range_low;
                                    part_high.x = range_high;
                                }
                                'm' => {
                                    part_low.m = range_low;
                                    part_high.m = range_high;
                                }
                                'a' => {
                                    part_low.a = range_low;
                                    part_high.a = range_high;
                                }
                                's' => {
                                    part_low.s = range_low;
                                    part_high.s = range_high;
                                }
                                _ => unreachable!(),
                            };

                            // info!(?part_low, ?part_high);

                            ApplyResult::SplitRange {
                                pass: (part_high, target),
                                fail: part_low,
                            }
                        }
                        _ => unreachable!(),
                    }
                } else {
                    if (part_range.end() < number && ordering == &Ordering::Less)
                        || (part_range.start() > number && ordering == &Ordering::Greater)
                    {
                        ApplyResult::Pass(target)
                    } else {
                        ApplyResult::Fail
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Part {
    x: RangeInclusive<usize>,
    m: RangeInclusive<usize>,
    a: RangeInclusive<usize>,
    s: RangeInclusive<usize>,
}

impl Default for Part {
    fn default() -> Self {
        Self {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
        }
    }
}

#[tracing::instrument(skip(input))]
fn target<'a>(input: &'a str) -> IResult<&str, Target<'a>> {
    alt((
        map(complete::char('A'), |_| Target::Accept),
        map(complete::char('R'), |_| Target::Reject),
        map(alpha1, |name| Target::Workflow(name)),
    ))(input)
}

#[tracing::instrument(skip(input))]
fn rule(input: &str) -> IResult<&str, Rule> {
    alt((
        map(
            tuple((
                one_of("xmas"),
                one_of("<>"),
                complete::i32,
                tag(":"),
                target,
            )),
            |(category, ordering, number, _, target)| {
                let ordering = match ordering {
                    '<' => Ordering::Less,
                    '>' => Ordering::Greater,
                    _ => unreachable!(),
                };

                Rule::Test {
                    category,
                    ordering,
                    number: number as usize,
                    target,
                }
            },
        ),
        map(target, |target| Rule::Target(target)),
    ))(input)
}

#[tracing::instrument(skip(input))]
fn rules(input: &str) -> IResult<&str, Vec<Rule>> {
    separated_list1(tag(","), rule)(input)
}

#[tracing::instrument(skip(input))]
fn workflow(input: &str) -> IResult<&str, (&str, Vec<Rule>)> {
    tuple((alpha1, delimited(tag("{"), rules, tag("}"))))(input)
}

#[tracing::instrument(skip(input))]
fn workflows(input: &str) -> IResult<&str, BTreeMap<&str, Vec<Rule>>> {
    map(separated_list1(line_ending, workflow), |workflows| {
        BTreeMap::from_iter(workflows)
    })(input)
}

#[tracing::instrument(skip(part, workflows, next_target))]
fn propagate_parts(
    part: Part,
    workflows: &BTreeMap<&str, Vec<Rule>>,
    next_target: &Target,
) -> usize {
    match next_target {
        // easy one: if a part if rejected, it counts as 0 for the
        // total sum
        Target::Reject => 0,

        // also easy: if a part is accepted we map it to the product
        // of the sizes of its ranges
        Target::Accept => {
            info!(?part);

            (part.x.end() - part.x.start() + 1)
                * (part.m.end() - part.m.start() + 1)
                * (part.a.end() - part.a.start() + 1)
                * (part.s.end() - part.s.start() + 1)
        }

        // if we have a target workflow, we recursively apply the part
        // to each of the rules in the workflow, accumulating the sum.
        Target::Workflow(id) => {
            let current_workflow = workflows.get(id).expect("should have valid workflow");
            let mut current_part = part;
            let mut sum = 0;

            for rule in current_workflow.iter() {
                match rule.apply(&current_part) {
                    ApplyResult::SplitRange { pass, fail } => {
                        sum += propagate_parts(pass.0, workflows, pass.1);
                        current_part = fail;
                    }
                    ApplyResult::Pass(target) => {
                        sum += propagate_parts(current_part.clone(), workflows, target);
                    }
                    ApplyResult::Fail => {}
                }
            }

            sum
        }
    }
}

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    info!("processing input");

    let (_, workflows) = workflows(input)?;

    let part = Part::default();
    let result = propagate_parts(part, &workflows, &Target::Workflow("in"));

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
        let input = r##"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"##;
        let result = process(input).unwrap();
        assert_eq!(result, "167409079868000");
    }
}
