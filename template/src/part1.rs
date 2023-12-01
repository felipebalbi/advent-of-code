use anyhow::{Context, Result};

#[tracing::instrument]
fn process(_input: &str) -> Result<String> {
    Ok("".to_string())
}

#[tracing::instrument]
pub fn part1() -> Result<()> {
    let file = include_str!("../input1.txt");
    let result = process(file).context("process part 1")?;
    println!("Part 1: {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = r##""##;
        let result = process(input).unwrap();
        assert_eq!(result, "42");
    }
}
