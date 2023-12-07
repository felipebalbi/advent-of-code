use anyhow::Result;
use day7::{part1, part2};

#[tracing::instrument]
fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let input = include_str!("../input.txt");
    let result = part1(input)?;
    println!("Part 1: {result}");

    let result = part2(input)?;
    println!("Part 2: {result}");

    Ok(())
}
