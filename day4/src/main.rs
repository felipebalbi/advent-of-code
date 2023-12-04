use anyhow::Result;
use day4::{part1, part2};

#[tracing::instrument]
fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let input1 = include_str!("../input1.txt");
    let result = part1(input1)?;
    println!("Part 1: {result}");

    let input2 = include_str!("../input2.txt");
    let result = part2(input2)?;
    println!("Part 2: {result}");

    Ok(())
}
