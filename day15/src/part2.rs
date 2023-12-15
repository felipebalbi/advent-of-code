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
    map(pair(label, operation), |(label, operation)| Instruction {
        label,
        operation,
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

    let mut boxes = BTreeMap::<usize, Vec<Lens>>::new();

    for instruction in instructions.iter() {
        let box_index = instruction.label.chars().fold(0, |mut acc, c| {
            acc += c as u8 as usize;
            acc *= 17;
            acc %= 256;

            acc
        });

        match instruction.operation {
            Operation::Pop => {
                if let Some(contents) = boxes.get_mut(&box_index) {
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
                    .entry(box_index)
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
    }

    let result = boxes.iter().fold(0, |acc, (index, box_map)| {
        acc + box_map.iter().enumerate().fold(0, |box_acc, (slot, lens)| {
            let focal_length = lens.focal_length;

            info!(?box_acc, ?index, ?slot, ?focal_length);
            box_acc + (index + 1) * (slot + 1) * focal_length as usize
        })
    });

    // rn: 1 (box 0) * 1 (first slot) * 1 (focal length) = 1
    // cm: 1 (box 0) * 2 (second slot) * 2 (focal length) = 4
    // ot: 4 (box 3) * 1 (first slot) * 7 (focal length) = 28
    // ab: 4 (box 3) * 2 (second slot) * 5 (focal length) = 40
    // pc: 4 (box 3) * 3 (third slot) * 6 (focal length) = 72

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
