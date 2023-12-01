use anyhow::Result;
use {{project-name}}::{part1, part2};

#[tracing::instrument]
fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    part1()?;
    part2()?;

    Ok(())
}
