use anyhow::Result;
use day1::{part1, part2};

#[tracing::instrument]
fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    part1()?;
    part2()?;

    Ok(())
}
