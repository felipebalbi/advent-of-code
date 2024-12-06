use anyhow::Result;
use day7::{part1, part2};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tracing::instrument]
fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = include_str!("../input.txt");
    let result = part1(input)?;
    println!("Part 1: {result}");

    let result = part2(input)?;
    println!("Part 2: {result}");

    Ok(())
}
