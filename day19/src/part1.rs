use anyhow::{Context, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, one_of},
    combinator::{map, opt},
    multi::{many1, separated_list1},
    sequence::{delimited, pair, separated_pair, tuple},
    IResult,
};
use std::{cmp::Ordering, collections::BTreeMap};
use tracing::info;

#[derive(Debug)]
enum Target<'a> {
    Accept,
    Reject,
    Workflow(&'a str),
}

#[derive(Debug)]
struct Rule<'a> {
    category: Option<char>,
    ordering: Option<Ordering>,
    number: Option<i32>,
    target: Target<'a>,
}

#[derive(Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

#[tracing::instrument(skip(input))]
fn target<'a>(input: &'a str) -> IResult<&str, Target<'a>> {
    alt((
        map(complete::char('A'), |_| Target::Accept),
        map(complete::char('R'), |_| Target::Reject),
        map(alpha1, |name| Target::Workflow(name)),
    ))(input)
}

// px{a<2006:qkq,m>2090:A,rfg}
// pv{a>1716:R,A}
// lnx{m>1548:A,A}
// rfg{s<537:gd,x>2440:R,A}

#[tracing::instrument(skip(input))]
fn rule(input: &str) -> IResult<&str, Rule> {
    map(
        tuple((
            opt(one_of("xmas")),
            opt(one_of("<>")),
            opt(complete::i32),
            opt(tag(":")),
            target,
        )),
        |(category, operation, number, _, target)| {
            let ordering = match operation {
                Some('<') => Some(Ordering::Less),
                Some('>') => Some(Ordering::Greater),
                _ => None,
            };

            Rule {
                category,
                ordering,
                number,
                target,
            }
        },
    )(input)
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

#[tracing::instrument(skip(input))]
fn xmas(input: &str) -> IResult<&str, (i32, i32, i32, i32)> {
    map(
        tuple((
            tag("x="),
            complete::i32,
            tag(",m="),
            complete::i32,
            tag(",a="),
            complete::i32,
            tag(",s="),
            complete::i32,
        )),
        |(_, x, _, m, _, a, _, s)| (x, m, a, s),
    )(input)
}

#[tracing::instrument(skip(input))]
fn part(input: &str) -> IResult<&str, Part> {
    map(delimited(tag("{"), xmas, tag("}")), |(x, m, a, s)| Part {
        x,
        m,
        a,
        s,
    })(input)
}

#[tracing::instrument(skip(input))]
fn parts(input: &str) -> IResult<&str, Vec<Part>> {
    separated_list1(line_ending, part)(input)
}

#[tracing::instrument(skip(input))]
fn workflows_and_parts(input: &str) -> IResult<&str, (BTreeMap<&str, Vec<Rule>>, Vec<Part>)> {
    separated_pair(workflows, many1(line_ending), parts)(input)
}

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    info!("processing input");

    let (_, (workflows, parts)) = workflows_and_parts(input)?;

    let mut accepted: Vec<Part> = vec![];

    for part in parts.iter() {
        let mut workflow = workflows.get("in").expect("should exist");

        for rule in workflow.iter() {
            match rule.category {
                Some('x') => todo!(),
                Some('m') => todo!(),
                Some('a') => todo!(),
                Some('s') => todo!(),
                _ => None::<char>,
            };
        }
    }

    info!(?workflows, ?parts);

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
        assert_eq!(result, "19114");
    }
}
