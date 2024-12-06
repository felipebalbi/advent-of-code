use anyhow::{Context, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1},
    combinator::map,
    multi::separated_list1,
    sequence::pair,
    IResult,
};
use std::collections::BTreeMap;
use tracing::info;

#[derive(Debug)]
enum Operation {
    Pop,
    Push(u32),
}

#[derive(Debug)]
struct Instruction<'a> {
    label: &'a str,
    hash: u8,
    operation: Operation,
}

#[derive(Debug, PartialEq, Clone)]
struct Lens<'a> {
    label: &'a str,
    focal_length: u32,
}

#[tracing::instrument(skip(input))]
fn label(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}

#[tracing::instrument(skip(input))]
fn operation(input: &str) -> IResult<&str, Operation> {
    alt((
        map(tag("-"), |_| Operation::Pop),
        map(pair(tag("="), complete::u32), |(_, value)| {
            Operation::Push(value)
        }),
    ))(input)
}

#[tracing::instrument(skip(input))]
fn instruction(input: &str) -> IResult<&str, Instruction> {
    map(pair(label, operation), |(label, operation)| {
        let hash: u8 = label
            .chars()
            .fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
            .try_into()
            .expect("should fit into a u8");

        Instruction {
            label,
            hash,
            operation,
        }
    })(input)
}

#[tracing::instrument(skip(input))]
fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(tag(","), instruction)(input)
}

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    info!("processing input");

    let (_, instructions) = instructions(input)?;

    let boxes = instructions.iter().fold(
        BTreeMap::<u8, Vec<Lens>>::new(),
        |mut boxes, instruction| {
            match instruction.operation {
                Operation::Pop => {
                    if let Some(contents) = boxes.get_mut(&instruction.hash) {
                        if let Some(index) = contents
                            .iter()
                            .position(|lens| lens.label == instruction.label)
                        {
                            contents.remove(index);
                        }
                    }
                }
                Operation::Push(focal_length) => {
                    let lens = Lens {
                        label: instruction.label,
                        focal_length,
                    };

                    boxes
                        .entry(instruction.hash)
                        .and_modify(|entry| {
                            if let Some(index) = entry
                                .iter()
                                .position(|lens| lens.label == instruction.label)
                            {
                                entry[index] = lens.clone();
                            } else {
                                (*entry).push(lens.clone());
                            }
                        })
                        .or_insert(vec![lens]);
                }
            }

            boxes
        },
    );

    let result = boxes.iter().fold(0, |acc, (index, box_map)| {
        acc + box_map.iter().enumerate().fold(0, |box_acc, (slot, lens)| {
            let focal_length = lens.focal_length;

            info!(?box_acc, ?index, ?slot, ?focal_length);
            box_acc + (*index as usize + 1) * (slot + 1) * focal_length as usize
        })
    });

    info!(?result);

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
        let input = r##"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"##;
        let result = process(input).unwrap();
        assert_eq!(result, "145");
    }
}
